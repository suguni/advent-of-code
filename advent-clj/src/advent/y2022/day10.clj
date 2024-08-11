(ns advent.y2022.day10
  (:require [clojure.string :as str]))

(defn noop [state]
  (update state :cycles conj (:x state)))

(defn add-x [n state]
  (let [{:keys [x cycles]} state]
    (assoc state :x (+ x n)
                 :cycles (vec (concat cycles [x x])))))

(defn execute [state lines]
  (if (empty? lines)
    state
    (let [line (first lines)
          state (if (re-matches #"noop" line)
                  (noop state)
                  (if-let [[_ n] (re-matches #"addx (-?\d+)" line)]
                    (add-x (Integer/parseInt n) state)
                    "TILT"))]
      state)))

(defn execute-file [file-name]
  (let [lines (str/split-lines (slurp file-name))
        starting-state {:x 1 :cycles {}}
        ending-state (execute starting-state lines)]
    (:cycles ending-state)))

(defn render-cycles [cycles]
  )

(denf print-screen [lines]
      )

(defn run [file-name]
  (-> file-name
      execute-file
      render-cycles
      print-screen))
