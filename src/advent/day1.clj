(ns advent.day1
  (:require [clojure.string :as str]
            [clojure.math.combinatorics :as comb]))

(defn all-pair [lst]
  (for [a lst
        b lst]
    [a b]))

(defn all-triple [lst]
  (for [a lst
        b lst
        c lst]
    [a b c]))

(defn day1-data [path]
  (->> (str/split (slurp path) #"\s")
       (map #(Integer/parseInt %))))

(defn day1-p1 [path]
  (first
   (filter (fn [[a b]] (= (+ a b) 2020))
           (comb/combinations (day1-data path) 2))))

(let [[a b] (day1-p1 "resources/day1-input")]
  (* a b))

(defn day1-p2 [path]
  (first
   (filter (fn [[a b c]] (= (+ a b c) 2020))
           (comb/combinations (day1-data path) 3))))

(let [[a b c] (day1-p2 "resources/day1-input")]
  (* a b c))
