(ns advent2020.day7-test
  (:require  [clojure.test :refer :all]
             [advent2020.day7 :refer :all]))


(deftest rule-match
  (is (re-find re-rule-bag
              "light red bags contain 1 bright white bag, 2 muted yellow bags."))
  (is (re-find re-rule-bag
              "bright white bags contain 1 shiny gold bag."))
  (is (re-find re-rule-bag
              "muted yellow bags contain 2 shiny gold bags."))
  (is (re-find re-rule-bag
              "faded blue bags contain no other bags.")))


(deftest data-loading
  (testing "parse-rule"
    (is (= {"dotted black" {}}
           (parse-rule "dotted black bags contain no other bags.")))

    (is (= {"light red" { "bright white" 1, "muted yellow" 2}}
           (parse-rule "light red bags contain 1 bright white bag, 2 muted yellow bags.")))
    ))
