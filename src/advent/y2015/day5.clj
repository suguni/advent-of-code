(ns advent.y2015.day5
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(def VOWEL (set "aeiou"))

(defn p1? [s]
  (->> s
       (filter VOWEL)
       count
       (< 2)))

(p1? "ugknbfddgicrmopn")
(p1? "aaa")

(defn p2? [s]
  (->> s
       (partition 2 1)
       (some #(apply = %))))

(p2? "ugknbfddgicrmopn")

(defn p3? [s]
  (not
    (some #(str/includes? s %) ["ab" "cd" "pq" "xy"])))

(p3? "haegwjzuvuyypxyu")

(defn p-all? [s]
  (and (p1? s) (p2? s) (p3? s)))

(p-all? "aaa")

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         vec)))

(def F "resources/2015/day5-input.txt")

(defn day5-p1 []
  (->> F
       load-data
       (filter p-all?)
       count))

(day5-p1)

(defn t1? [s]
  (and (->> s
            (partition 2 1)
            frequencies
            vals
            (some #(> % 1)))
       (->> s
            (partition 3 1)
            (some #(apply = %))
            not)))

(t1? "aabcdefgaa")
(t1? "xyxy")

(defn t2? [s]
  (->> s
       (partition 3 1)
       (some (fn [[a _ c]] (= a c)))))

(defn t-all? [s]
  (and (t1? s) (t2? s)))

(t-all? "qjhvhtzxzqqjkmpb")
(t-all? "xxyxx")
(t-all? "uurcxstgmygtbstg")
(t-all? "ieodomkazucvgmuy")

(defn day5-p2 []
  (->> F
       load-data
       (filter t-all?)
       count))

(day5-p2)