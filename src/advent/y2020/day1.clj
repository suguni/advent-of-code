(ns advent.y2020.day1
  (:require [clojure.string :as str]
            [clojure.math.combinatorics :as comb]
            [clojure.java.io :as io]))


(def FILE "resources/2020/day1-input.txt")

(defn read-numbers [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map #(Integer/parseInt %))
         set)))

(comment
  (defn read-numbers [filename]
    (->> filename
         slurp
         str/split-lines
         (map #(Integer/parseInt %)))))

(defn two-thousand-tuple [n numbers]
  (->> (comb/combinations numbers n)
       (filter #(= (apply + %) 2020))
       first))

(defn day1 [filename n]
  (->> filename
       read-numbers
       (two-thousand-tuple n)
       (apply *)))

(day1 FILE 2)
(day1 FILE 3)
