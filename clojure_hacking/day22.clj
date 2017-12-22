(def input (clojure.string/split-lines (slurp "d22_input.txt")))
(def height (count input))
(def width (count (first input)))
(def init-pos [(int (/ width 2)) (int (/ height 2))])

(defn- parse-line [y]
       (->> (nth input (- height y 1)) ; y=0 is bottom of the input file
            (map-indexed (fn [x ch]
                             [[x y] (if (= ch \#) :infected)]))
            (remove #(nil? (second %))) ; sparse map - don't store clean coords
            (into {})))

(def init-state
  (loop [y (dec height)
         state {}]
        (if (neg? y)
          state
          (recur (dec y) (merge state (parse-line y))))))

(def opposite {:down :up :up :down :left :right :right :left})
(def ccw {:down :right :right :up :up :left :left :down})
(def cw {:down :left :left :up :up :right :right :down})

(defn- move [[x y] dir]
       (case dir
             :down [x (dec y)]
             :up [x (inc y)]
             :left [(dec x) y]
             :right [(inc x) y]))

(defn- step-p1 [{:keys [dir pos state infect]}]
       (let [was-infected (= :infected (state pos :clean))
             new-dir (if was-infected (cw dir) (ccw dir))
             new-state (if was-infected (dissoc state pos) (assoc state pos :infected))
             new-pos (move pos new-dir)
             new-infect (if (not was-infected) (inc infect) infect)]

            {:dir new-dir :pos new-pos :state new-state :infect new-infect}))

(defn- step-p2 [{:keys [dir pos state infect]}]
       (let [status (state pos :clean)
             new-dir (case status
                           :clean (ccw dir)
                           :weakened dir
                           :infected (cw dir)
                           :flagged (opposite dir))
             new-state (case status
                             :clean (assoc state pos :weakened)
                             :weakened (assoc state pos :infected)
                             :infected (assoc state pos :flagged)
                             :flagged (dissoc state pos))
             new-pos (move pos new-dir)
             new-infect (if (= :weakened status) (inc infect) infect)]

            {:dir new-dir :pos new-pos :state new-state :infect new-infect}))

(time (println "part 1:" (:infect (first (drop 10000 (iterate step-p1 {:dir :up :pos init-pos :state init-state :infect 0}))))))
(time (println "part 2:" (:infect (first (drop 10000000 (iterate step-p2 {:dir :up :pos init-pos :state init-state :infect 0}))))))
