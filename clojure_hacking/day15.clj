(def ^:const A_FACTOR 16807)
(def ^:const B_FACTOR 48271)
(def ^:const A_START 512)
(def ^:const B_START 191)
(def ^:const DIVISOR 2147483647)

(defn- good? [[a b]]
       (= (bit-and a 0xFFFF) (bit-and b 0xFFFF)))

; the two seqs are inlined because using def/let causes the head to be retained, which consumes too much memory
(println "part 1:" (->> (map vector
                             (next (iterate #(rem (* % A_FACTOR) DIVISOR) A_START))
                             (next (iterate #(rem (* % B_FACTOR) DIVISOR) B_START)))
                        (take 40000000)
                        (filter good?)
                        (count)))

(println "part 2:" (->> (map vector
                             (filter #(zero? (bit-and 3 %)) (next (iterate #(rem (* % A_FACTOR) DIVISOR) A_START)))
                             (filter #(zero? (bit-and 7 %)) (next (iterate #(rem (* % B_FACTOR) DIVISOR) B_START))))
                        (take 5000000)
                        (filter good?)
                        (count)))
