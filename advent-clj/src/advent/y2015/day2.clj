(ns advent.y2015.day2
  (:require [clojure.string :as str]
            [clojure.java.io :as io]))

(defn parse-dims [dims]
  (->> (str/split dims #"x")
       (map #(Integer/parseInt %))))

(defn surface-area [[w h l]]
  (* 2 (+ (* l w) (* w h) (* h l))))

(defn extra-area [[w h l]]
  (min (* l w) (* w h) (* h l)))

(defn require-paper-area [whl]
  (+ (surface-area whl)
     (extra-area whl)))

(defn wrap-length [[w h l]]
  (* 2 (min (+ l w) (+ w h) (+ h l))))

(defn bow-length [[w h l]]
  (* w h l))

(defn require-ribbon-length [whl]
  (+ (wrap-length whl)
     (bow-length whl)))

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         (map parse-dims)
         (into []))))

(def filename "resources/2015/day2-input.txt")

(defn day2-p1 []
  (->> filename
       load-data
       (map require-paper-area)
       (reduce + 0)))


(defn day2-p2 []
  (->> filename
       load-data
       (map require-ribbon-length)
       (reduce + 0)))
