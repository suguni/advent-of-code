(ns advent.day1
  (:require [clojure.string :as str]))

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
  (->> path
       day1-data
       all-pair
       (filter (fn [[a b]] (= (+ a b) 2020)))
       first))

(let [[a b] (day1-p1 "resources/input")]
  (* a b))

(defn day1-p2 [path]
  (->> path
       day1-data
       all-triple
       (filter (fn [[a b c]] (= (+ a b c) 2020)))
       first))

(let [[a b c] (day1-p2 "resources/input")]
  (* a b c))
