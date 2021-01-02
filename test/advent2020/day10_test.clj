(ns advent2020.day10-test
  (:require [advent2020.day10 :as sut]
            [clojure.test :as t]))


(t/deftest solve-part1
  (t/is (= (sut/part1 sut/input1)
           {1 7, 3 5})))

(t/deftest under-3
  (t/is (= (sut/under-3-pair 0 [1 2 3])
           [[1 [2 3]]
            [2 [3]]
            [3 []]])
        (= (sut/under-3-pair 0 [1 3 4 5])
           [[1 [3 4 5]]
            [3 [4 5]]])))

(t/deftest arrangement
  (t/is (= (sut/slow-arrangement 0 [1 2 3])
           [[0 1 2 3]
            [0 1 3]
            [0 2 3]
            [0 3]]))
  (t/is (= (sut/slow-arrangement 0 [2 4])
           [[0 2 4]])))
