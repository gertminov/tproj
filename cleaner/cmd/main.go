package main

import (
	"flag"
	"github.com/aerogo/log"
	"os"
	"path/filepath"
	"time"
)

func main() {
	path := ""
	flag.StringVar(&path, "p", "", "path of temp project directory")
	flag.Parse()

	logging := log.New()
	logging.AddWriter(log.File(path + "/log.log"))
	logging.Info("------------------------------")
	t := time.Now()
	logging.Info("Started cleaning at: " + t.Format(time.RFC3339))

	projectDirs, err := os.ReadDir(path)
	if err != nil {
		logging.Error("error opening directory: %v", err)
	}
	for _, projectDir := range projectDirs {
		if projectDir.IsDir() {
			projectPath := filepath.Join(path, projectDir.Name())

			workingDir := filepath.Join(projectPath, "working")
			err := os.RemoveAll(workingDir)
			if err != nil {
				logging.Error("could not remove working dir of project: %v\nerror: %v", projectDir.Name(), err)
			}

			outPath := filepath.Join(projectPath, "out")
			entries, err := os.ReadDir(outPath)
			if err != nil {
				logging.Error("could not read 'out' dir of %v", projectDir.Name())
			}

			if len(entries) == 0 {
				projectPath := filepath.Join(path, projectDir.Name())
				err := os.RemoveAll(projectPath)
				if err != nil {
					logging.Error("Could not remove project: %v", projectDir.Name())
				} else {
					logging.Info("removed project: %v", projectDir.Name())
				}
			} else {
				logging.Info("project: %v has files in out dir")
			}
		}
	}

	t = time.Now()
	logging.Info("ended cleaning: %v", t.Format(time.RFC3339))
	time.Sleep(1 * time.Second)
}
