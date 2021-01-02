(ns advent2020.day9-test
  (:require [advent2020.day9 :as sut]
            [clojure.test :as t]))

(t/deftest check-contain-sum-of-two
  (t/is (not (sut/check (range 1 26) 1)))
  (t/is (sut/check (range 1 26) 3))
  (t/is (sut/check (range 1 26) (+ 24 25)))
  (t/is (sut/check [35 20 15 25 47] 40)))

(def numbers [35 20 15 25 47 40 62 55 65 95 102 117
              150 182 127 219 299 277 309 576])

(t/deftest check-list-of-numbers
   (t/is (= (sut/invalid-first-number 5 numbers) 127))
  )

(t/deftest find-continuous-range-sum-value
  (t/is (= (sut/cont-list-sum-value numbers 127) [15 25 47 40])))
