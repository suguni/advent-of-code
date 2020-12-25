(ns advent.day16
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(def small "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,13")

(defn parse-ints [line]
   (->> line
        (re-seq #"\d+")
        (map #(Integer/parseInt %))))

(defn parse-references [refs]
  (let [[rule-block yours-block nearby-block]
        (str/split refs #"\n\n")
        rules (->> rule-block
                   str/split-lines
                   (map parse-ints)
                   (map #(partition 2 %)))
        yours (->> yours-block
                   str/split-lines
                   second
                   parse-ints)
        nearby (->> nearby-block
                    str/split-lines
                    rest
                    (map parse-ints))]
    [rules yours nearby]))

(def sn
  (parse-references small))

(defn between [n [[a b] [d e]]]
  (or (<= a n b) (<= d n e)))

(defn scanning-error [[rules _ nearby]]
  (->> nearby
       flatten
       (remove (fn [ticket]
                 (some #(between ticket %) rules)))
       (apply +)))

(defn remove-error-nearby [[rules _ nearby]]
  (->> nearby
       (filter (fn [tickets]
                 (every? (fn [ticket] (some #(between ticket %) rules)) tickets)))))

(defn transpose [vecs]
  (apply map vector vecs))

(defn common-rules [rules tickets]
  (->> rules
       (map vector (range (count rules)))
       (filter (fn [[_ slot]]
                 (every? #(between % slot) tickets)))
       (map first)))

(common-rules
 [[[0 1] [4 19]] [[0 5] [8 19]] [[0 3] [16 19]]]
 [3 15 5])

(let [[rules your nearby :as note]
      (parse-references small)]
  (->> note
       remove-error-nearby
       transpose
       (map #(common-rules rules %))))


(def filename "resources/day16-input.txt")

(let [[rules your _ :as note] (->> filename
                                   slurp
                                   parse-references)
      candidates (->> note
                      remove-error-nearby
                      (cons your)
                      transpose
                      (map #(common-rules rules %))
                      (map vector (iterate inc 0)))]

  (->> (loop [c candidates
              r {}]
         (if (seq c)
           (let [cs (sort-by (fn [[_ l]] (count l)) c)
                 [i [v]] (first cs)
                 c2 (map (fn [[i ls]]
                           [i (remove #(= % v) ls)])
                         (rest cs))]
             (recur c2
                    (assoc r i v)))
           r))
       (filter (fn [[_ v]] (<= 0 v 5)))
       (map first)
       (map #(nth your %))
       (apply *)))
