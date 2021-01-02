(ns advent2020.day9
  (:require [clojure.math.combinatorics :as comb]
            [clojure.java.io :as io]))

(defn check [numbers value]
  (->> (comb/combinations numbers 2)
       (map #(apply + %))
       (filter #(= value %))
       first))

(defn invalid-first-number [window numbers]
  (->> numbers
       (partition (inc window) 1)
       (map #(split-at window %))
       (drop-while #(let [[nums [val]] %] (check nums val)))
       first
       last
       first
       ))

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map #(Long/parseLong %))
         vec)))

(def F "resources/2020/day9-input.txt")

;; (load-data F)

(->> F
     load-data
     (invalid-first-number 25))


(def numbers [35 20 15 25 47 40 62 55 65 95 102 117
              150 182 127 219 299 277 309 576])

(defn cont-list-sum-value [numbers value]
  (->> (range (count numbers))
       (map #(drop % numbers))
       (map (fn [lst]
              (->> (range 2 (count lst))
                   (map #(take % lst))
                   (drop-while #(< (apply + %) value))
                   first)))
       (filter #(= (apply + %) value))
       first))

(let [numbers (load-data F)
      value (invalid-first-number 25 numbers)
      sum-values (cont-list-sum-value numbers value)]
  (+ (apply min sum-values)
     (apply max sum-values)))
