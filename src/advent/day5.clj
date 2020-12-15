(ns advent.day5
  (:require [clojure.java.io :as io]))

(def FILENAME "resources/day5-input")

(defn seat-id [row col]
  (+ (* row 8) col))

(defn binary [low high line]
  (reduce (fn [[low high] s]
            (let [mid (int (/ (+ low high) 2))]
              (if s [(+ mid 1) high] [low mid])))
          [low high]
          line))

(defn row-num [line]
  (->> (subs line 0 7)
       (map #(= \B %))
       (binary 0 127)
       first))

(defn col-num [line]
  (->> (subs line 7)
       (map #(= \R %))
       (binary 0 7)
       first))

(col-num "BFFFBBFRLR")

(defn line-seat-id [line]
  (seat-id (row-num line) (col-num line)))

(defn seat-ids [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map line-seat-id)
         vec)))

(defn part1 []
  (apply max (seat-ids FILENAME)))

(defn part2
  (let [ordered-seats (sort (seat-ids FILENAME))]
    (->>  (map vector (rest ordered-seats) ordered-seats)
          (map (fn [[a b]] [(- a b) b]))
          (filter (fn [[diff id]] (not (= diff 1))))
          first
          second
          inc)))
