(ns advent.y2015.day2-test
  (:require [clojure.test :refer :all]
            [advent.y2015.day2 :as s]))

(deftest parse-dim
  (is (= (seq [2 3 4]) (s/parse-dims "2x3x4"))))

(deftest surface-area
  (is (= 52 (s/surface-area [2 3 4])))
  (is (= 42 (s/surface-area [1 1 10]))))

(deftest extra-area
  (is (= 6 (s/extra-area [2 3 4]))))

(deftest require-area
  (is (= 58 (s/require-paper-area "2x3x4")))
  (is (= 43 (s/require-paper-area "1x1x10"))))

