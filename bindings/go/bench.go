package main

/*
#cgo darwin LDFLAGS: -L${SRCDIR}/../../target/release -lsqlgrok -Wl,-rpath,${SRCDIR}/../../target/release
#cgo linux LDFLAGS: -L${SRCDIR}/../../target/release -lsqlgrok -Wl,-rpath,${SRCDIR}/../../target/release
#include <stdlib.h>

char *sqlgrok_transpile(const char *sql, const char *from_dialect, const char *to_dialect);
void sqlgrok_free(char *ptr);
*/
import "C"

import (
	"bufio"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"sort"
	"strings"
	"time"
	"unsafe"
)

type benchCase struct {
	ID    string `json:"id"`
	SQL   string `json:"sql"`
	Read  string `json:"read"`
	Write string `json:"write"`
}

type benchResult struct {
	Binding       string        `json:"binding"`
	Checksum      uint32        `json:"checksum"`
	Cases         int           `json:"cases"`
	Iterations    int           `json:"iterations"`
	Samples       int           `json:"samples"`
	Operations    int           `json:"operations"`
	MinNSPerOp    float64       `json:"min_ns_per_op"`
	MeanNSPerOp   float64       `json:"mean_ns_per_op"`
	MedianNSPerOp float64       `json:"median_ns_per_op"`
	P95NSPerOp    float64       `json:"p95_ns_per_op"`
	MaxNSPerOp    float64       `json:"max_ns_per_op"`
	PerSample     []benchSample `json:"per_sample"`
}

type benchSample struct {
	ElapsedNS int64   `json:"elapsed_ns"`
	NSPerOp   float64 `json:"ns_per_op"`
	Checksum  uint32  `json:"checksum"`
}

func readCases(path string) ([]benchCase, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var cases []benchCase
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		var c benchCase
		if err := json.Unmarshal([]byte(line), &c); err != nil {
			return nil, err
		}
		cases = append(cases, c)
	}
	return cases, scanner.Err()
}

func transpile(c benchCase) (string, error) {
	sql := C.CString(c.SQL)
	read := C.CString(c.Read)
	write := C.CString(c.Write)
	defer C.free(unsafe.Pointer(sql))
	defer C.free(unsafe.Pointer(read))
	defer C.free(unsafe.Pointer(write))

	ptr := C.sqlgrok_transpile(sql, read, write)
	if ptr == nil {
		return "", fmt.Errorf("transpile failed for %s", c.ID)
	}
	defer C.sqlgrok_free(ptr)
	return C.GoString(ptr), nil
}

func percentile(values []float64, q float64) float64 {
	if len(values) == 0 {
		return 0
	}
	sorted := append([]float64(nil), values...)
	sort.Float64s(sorted)
	index := int(q/100*float64(len(sorted))+0.999999999) - 1
	if index < 0 {
		index = 0
	}
	if index >= len(sorted) {
		index = len(sorted) - 1
	}
	return sorted[index]
}

func summarize(values []float64) (float64, float64, float64, float64, float64) {
	if len(values) == 0 {
		return 0, 0, 0, 0, 0
	}
	minValue := values[0]
	maxValue := values[0]
	sum := 0.0
	for _, value := range values {
		if value < minValue {
			minValue = value
		}
		if value > maxValue {
			maxValue = value
		}
		sum += value
	}
	return minValue, sum / float64(len(values)), percentile(values, 50), percentile(values, 95), maxValue
}

func main() {
	casesPath := flag.String("cases", "../../benchmarks/cases/postgres_sqlite.jsonl", "JSONL benchmark cases")
	iterations := flag.Int("iterations", 1000, "measured iterations")
	samplesFlag := flag.Int("samples", 5, "measured samples")
	warmup := flag.Int("warmup", 100, "warmup iterations")
	flag.Parse()

	cases, err := readCases(*casesPath)
	if err != nil {
		panic(err)
	}

	var checksum uint32
	for i := 0; i < *warmup; i++ {
		for _, c := range cases {
			out, err := transpile(c)
			if err != nil {
				panic(err)
			}
			checksum += uint32(len(out))
		}
	}

	samples := make([]benchSample, 0, *samplesFlag)
	var measuredChecksum uint32
	for sample := 0; sample < *samplesFlag; sample++ {
		checksum = 0
		started := time.Now()
		for i := 0; i < *iterations; i++ {
			for _, c := range cases {
				out, err := transpile(c)
				if err != nil {
					panic(err)
				}
				checksum += uint32(len(out))
			}
		}
		elapsed := time.Since(started)
		measuredChecksum = checksum
		samples = append(samples, benchSample{
			ElapsedNS: elapsed.Nanoseconds(),
			NSPerOp:   float64(elapsed.Nanoseconds()) / float64(len(cases)*(*iterations)),
			Checksum:  checksum,
		})
	}
	operations := len(cases) * *iterations
	nsPerOp := make([]float64, 0, len(samples))
	for _, sample := range samples {
		nsPerOp = append(nsPerOp, sample.NSPerOp)
	}
	minNS, meanNS, medianNS, p95NS, maxNS := summarize(nsPerOp)

	result := benchResult{
		Binding:       "go-cgo",
		Checksum:      measuredChecksum,
		Cases:         len(cases),
		Iterations:    *iterations,
		Samples:       *samplesFlag,
		Operations:    operations,
		MinNSPerOp:    minNS,
		MeanNSPerOp:   meanNS,
		MedianNSPerOp: medianNS,
		P95NSPerOp:    p95NS,
		MaxNSPerOp:    maxNS,
		PerSample:     samples,
	}
	encoded, err := json.Marshal(result)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(encoded))
}
