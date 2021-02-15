(ns advent.y2015.day1-test
  (:require [clojure.test :refer :all]
            [advent.y2015.day1 :as s]))

(deftest umm
  (is (= 0 (s/floor "(())")))
  (is (= 3 (s/floor "(()(()(")))
  (is (= 3 (s/floor "))((((("))))
