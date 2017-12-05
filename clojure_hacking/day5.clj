(def nums (with-open [rdr (clojure.java.io/reader "/home/vincent/CS/advent_of_code_2017/d5_input.txt")]
                      (->> (line-seq rdr)
                           (map #(Integer/parseInt %))
                           (into []))))

(defn- compute [nums part2]
       (loop [insns nums
              pc 0
              steps 0]
             (if (or (>= pc (count insns))
                       (< pc 0))
                   steps
                   (let [insn (insns pc)]
                        (recur (update insns pc (if (and part2 (>= insn 3)) dec inc))
                               (+ pc insn)
                               (inc steps))))))

(println "part 1:" (compute nums false))
(println "part 2:" (compute nums true))
