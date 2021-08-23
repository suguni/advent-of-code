(ns advent.y2020.day4
  (:require [clojure.java.io :as io]
            [clojure.string :as str]
            [clojure.set :as set]))

(def filename "resources/2020/day4-input.txt")

(defn concat-str [a b]
  (if (= a "") b (str a " " b)))

(defn read-seq [seq]
  (reduce (fn [acc line]
            (if (str/blank? line)
              (conj acc "")
              (conj (pop acc) (concat-str (last acc) line))))
          [""] seq))

(defn process-line [line]
  (->> (str/split line #"\s+")
       (map #(str/split % #":"))
       (map (fn [[a b]] [(keyword a) b]))
       (into (hash-map))))

(defn load-data [filename]
  (with-open [rdr (io/reader filename)]
    (->> rdr
         line-seq
         read-seq
         (map process-line))))

(process-line "hgt:138 ecl:grn pid:21019503 eyr:1937 byr:2008 hcl:z")

(def required-fields
  #{:byr :iyr :eyr :hgt :hcl :ecl :pid})

(def optional-fields
  #{:cid})

(defn rule1 [record]
  (let [all-fields (set/union required-fields optional-fields)]
    (->> record keys set
         (set/difference all-fields)
         (set/superset? optional-fields))))

(defn rule1 [{:keys [byr iyr eyr hgt hcl ecl pid] :as record}]
  (and byr iyr eyr hgt hcl ecl pid))

(defn numeric-check [s l h]
  (try
    (<= l (Integer/parseInt s) h)
    (catch Exception _ false)))

(def re-height #"^(\d+)(cm|in)$")

(defn height-check [s]
  (if-let [[_ height unit] (re-find re-height s)]
    (if (= unit "cm")
      (numeric-check height 150 193)
      (numeric-check height 59 76))))

(def re-hair-color #"^#[0-9a-f]{6}$")
(def re-eye-color #"^(amb|blu|brn|gry|grn|hzl|oth)$")
(def re-passport-id #"^\d{9}$")

(defn rule2 [{:keys [byr iyr eyr hgt hcl ecl pid] :as record}]
  (and byr (numeric-check byr 1920 2002)
       iyr (numeric-check iyr 2010 2020)
       eyr (numeric-check eyr 2020 2030)
       hgt (height-check hgt)
       hcl (re-find re-hair-color hcl)
       ecl (re-find re-eye-color ecl)
       pid (re-find re-passport-id pid)))

(defn solve [rule]
  (->> (load-data filename)
       (filter rule)
       count))

(solve rule1)
(solve rule2)
