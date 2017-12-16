(def ranges (with-open [rdr (clojure.java.io/reader "/home/vincent/CS/advent_of_code_2017/d13_input.txt")]
                      (->> (line-seq rdr)
                           (mapcat #(clojure.string/split % #": "))
                           (map #(Integer/parseInt %))
                           (apply hash-map))))

; returns severity of collision, nil if no collision
(defn- severity [ranges delay]
       (let [severities (->> ranges
                             (filter #(zero? (mod (+ (key %) delay)
                                                  (* 2 (dec (val %))))))
                             (map #(* (key %) (val %))))]
            (if (empty? severities)
              nil
              (reduce + severities))))

(println "part 1:" (severity ranges 0))

(println "part 2:"
         (->> (range)
              (map (fn [delay] [delay (severity ranges delay)])) ; todo: is there a smarter way to hold on to the index?
              (drop-while #(second %))
              (ffirst)))
