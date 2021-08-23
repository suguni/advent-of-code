(ns advent.y2015.day3-test
  (:require [clojure.test :refer :all]
            [advent.y2015.day3 :as s]))

(deftest step
  (is (= (s/step [0 0] \>) [1 0]))
  (is (= (s/step [0 0] \^) [0 1]))
  (is (= (s/step [0 0] \<) [-1 0]))
  (is (= (s/step [0 0] \v) [0 -1])))

(deftest move
  (is (= (s/move ">") [[0 0] [1 0]]))
  (is (= (s/move "^>v<") [[0 0] [0 1] [1 1] [1 0] [0 0]]))
  (is (= (s/move "^v^v") [[0 0] [0 1] [0 0] [0 1] [0 0]])))

(deftest pair-move
  (is (= (s/pair-move "^v") [[[0 0] [0 1]] [[0 0] [0 -1]]]))
  (is (= (s/pair-move "^>v<") [[[0 0] [0 1] [0 0]] [[0 0] [1 0] [0 0]]]))
  (is (= (s/pair-move "^v^v") [[[0 0] [0 1] [0 2]] [[0 0] [0 -1] [0 -2]]])))
