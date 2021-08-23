(ns advent.y2015.day4
  (:require [clojure.string :as str])
  (:import (java.security MessageDigest)
           (java.math BigInteger)))

(def input "bgvyzdsv")

(defn md5 [^String s]
  "from https://gist.github.com/jizhang/4325757"
  (let [algorithm (MessageDigest/getInstance "MD5")
        raw (.digest algorithm (.getBytes s))]
    (format "%032x" (BigInteger. 1 raw))))

(defn padded-md5-lowest-number [input pad]
  (->> (range)
       (map #(vector (md5 (str input %)) %))
       (filter #(str/starts-with? (first %) pad))
       first
       second))

(defn day4-p1 []
  (padded-md5-lowest-number input "00000"))

(defn day4-p2 []
  (padded-md5-lowest-number input "000000"))
