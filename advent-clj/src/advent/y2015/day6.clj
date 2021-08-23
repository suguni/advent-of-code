(ns advent.y2015.day6
  (:require [clojure.java.io :as io]))

(set! *warn-on-reflection* true)

(def SIZE 1000)

(defn parse [s]
  (let [[_ t sx sy ex ey]
        (re-matches
          #"(toggle|turn on|turn off) (\d+)\,(\d+) through (\d+)\,(\d+)"
          s)]
    {:turn (case t
             "toggle" :toggle
             "turn on" :on
             "turn off" :off)
     :s    [(Integer/parseInt sx) (Integer/parseInt sy)]
     :e    [(Integer/parseInt ex) (Integer/parseInt ey)]}))

(defn control1 [^Integer current turn]
  (case turn
    :on 1
    :off 0
    :toggle (mod (+ current 1) 2)))

(defn control2 [^Integer current turn]
  (case turn
    :on (inc current)
    :off (max 0 (dec current))
    :toggle (+ 2 current)))

(defn turn [^ints grid
            ^Integer size
            {turn :turn [sx sy] :s [ex ey] :e}
            controller]
  (doseq [^Integer x (range sx (inc ex))
          ^Integer y (range sy (inc ey))]
    (let [^Integer i (+ (* y size) x)]
      (aset-int grid i (controller (aget grid i) turn))))
  grid)

(def F "resources/2015/day6-input.txt")

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map parse)
         vec)))

#_(load-data F)

(defn process [filename]
  (->> filename
       load-data
       (reduce (fn [grid cmd] (turn grid SIZE cmd control1))
               (int-array (* SIZE SIZE) 0))
       vec
       (filter #(= % 1))
       count))

(defn process2 [filename]
  (->> filename
       load-data
       (reduce (fn [grid cmd] (turn grid SIZE cmd control2))
               (int-array (* SIZE SIZE) 0))
       vec
       (reduce +)))
