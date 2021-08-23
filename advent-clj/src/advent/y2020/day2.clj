(ns advent.y2020.day2
  (:require [clojure.string :as str]))

(defn read-line [line]
  (let [[_ b e c pwd]
        (re-find (re-matcher #"(\d+)-(\d+)\s+([a-z])\:\s+([a-z]+)" line))
        beg (Integer/parseInt b)
        end (Integer/parseInt e)
        ch (first c)]
    {:beg beg :end end :ch ch :pwd pwd}))

(defn policy1? [{beg :beg end :end ch :ch pwd :pwd}]
  (let [cnt (get (frequencies pwd) ch 0)]
    (and (>= cnt beg) (<= cnt end))))

(defn policy2? [{beg :beg end :end ch :ch pwd :pwd}]
  (and (>= (count pwd) end)
       (let [bc (get pwd (dec beg))
             ec (get pwd (dec end))]
         (and (or (= bc ch) (= ec ch))
              (not (= bc ec ch))))))

(policy1? (read-line "1-7 q: qqqqxvqrkbqqztlqlzq"))
(policy2? (read-line "1-3 q: cqbm"))


(defn day2 [file match-policy?]
  (->> file
      slurp
      str/split-lines
      (map read-line)
      (filter match-policy?)
      count))

(day2 "resources/2020/day2-input.txt" policy1?)
(day2 "resources/2020/day2-input.txt" policy2?)
