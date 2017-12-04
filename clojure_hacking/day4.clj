(def lines (with-open [rdr (clojure.java.io/reader "/home/vincent/CS/advent_of_code_2017/d4_input.txt")]
                      (->> (line-seq rdr)
                           (map #(clojure.string/split % #"\s"))
                           (into []))))

(println "part 1:" (count (filter #(apply distinct? %) lines)))

(let [freqs (map (fn [words]
                     (map frequencies words)) lines)]
     (println "part 2:" (count (filter #(apply distinct? %) freqs))))