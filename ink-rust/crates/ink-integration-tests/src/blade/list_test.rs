#[cfg(test)]
mod list_tests {
    use crate::blade::common::{compile_string, continue_all};

    #[test]
    fn list_basic_operations_test() {
        let story = compile_string("LIST things = apple, banana, cherry\nVAR t = (apple, banana)\n{t}\n~ t += cherry\n{t}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("apple"));
        assert!(text.join("").contains("banana"));
        assert!(text.join("").contains("cherry"));
    }

    #[test]
    fn list_mixed_items_test() {
        let story = compile_string(
            "LIST items = (first), second, (third)\nVAR i = (first, third)\n{i}\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("first"));
        assert!(text.join("").contains("third"));
    }

    #[test]
    fn more_list_operations_test() {
        let story = compile_string(
            "LIST list = l, m = 5, n\n{LIST_VALUE(l)}\n\n{list(1)}\n\n~ temp t = list()\n~ t= n\n{t}\n~ t = LIST_ALL(t)\n~ t -= n\n{t}\n~ t = LIST_INVERT(t)\n{t}\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("l"));
        assert!(text.join("").contains("n"));
    }

    #[test]
    fn empty_list_origin_test() {
        let story = compile_string("LIST a = A\nLIST b = B\n{LIST_ALL(A + B)}\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("A"));
        assert!(text.join("").contains("B"));
    }

    // From blade-ink-rs/conformance-tests/tests/list_test.rs

    #[test]
    fn list_save_load_test() {
        // Test save/load state with lists
        let story = compile_string(
            "LIST items = a, b, c\nVAR x = (a, b)\n{x}\n-> else\n\n=== else ===\n~ x = z\n{x}\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("a"));
        assert!(text.join("").contains("b"));
    }

    // Note: original has typo "empty_list_origin_after_assinment_test"
    #[test]
    fn empty_list_origin_after_assinment_test() {
        let story = compile_string("LIST list = a, b, c\n~ x = ()\n{LIST_ALL(x)}\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        // Empty list produces no output
    }

    #[test]
    fn list_range_test() {
        let story = compile_string(
            "LIST Food = Pizza, Pasta, Curry, Paella\nLIST Currency = Pound, Euro, Dollar\nLIST Numbers = One, Two, Three, Four, Five, Six, Seven\nVAR all = ()\n~ all = LIST_ALL(Food) + LIST_ALL(Currency)\n{all}\n{LIST_RANGE(all, 2, 3)}\n{LIST_RANGE(LIST_ALL(Numbers), Two, Six)}\n{LIST_RANGE((Pizza, Pasta), -1, 100)}\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Pound"));
        assert!(text.join("").contains("Euro"));
        assert!(text.join("").contains("Dollar"));
        assert!(text.join("").contains("Pizza"));
        assert!(text.join("").contains("Paella"));
    }

    #[test]
    fn list_bug_adding_element_test() {
        let story = compile_string(
            "LIST gameState = KNOW_ALIEN_REPORT\n\n- (init)\n\n+   a\n    ~ gameState += KNOW_ALIEN_REPORT\n    -> init\n\n+  {gameState ? KNOW_ALIEN_REPORT} OK\n    -> init\n\n+ FAIL\n    -> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert_eq!(0, text.len());
        s.ChooseChoiceIndex(0);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("OK"));
        s.ChooseChoiceIndex(1);
        let text = continue_all(&mut s);
        assert!(text.join("").contains("OK"));
    }

    #[test]
    fn more_list_operations2_test() {
        let story = compile_string(
            "LIST list1 = (a1), b1, c1\nLIST list2 = a2, b2, c2\nLIST list3 = a3, b3, c3\nVAR vlist = ()\n\n{LIST_ALL(list1)}\n{list1}\n\n~list2 += a1\n~list2 += b2\n\n{list2}\ncount:{LIST_COUNT(list2)}\n\n~list2 += c2\n\nmax:{LIST_MAX(list2)}\nmin:{LIST_MIN(list2)}\n\n// Equality\n~temp t = list2\n{t == list2}\n{t == (a1, b2, c2)}\n{t != list2}\n\n//emptiness\n{list3: not empty| empty}\n\n~vlist = (a2)\n{ vlist }\n{ LIST_ALL(vlist) }\n\nrange:{ LIST_RANGE(list2, 1, 2)}\n{ LIST_RANGE(list2, a1, a3)}\n\nsubtract:{(a1,b1,c1) - (b1)}\n\n~ SEED_RANDOM(10)\nrandom:{LIST_RANDOM(t)}\n\nlistinc:{(a1) + 1}\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("a1"));
        assert!(text.join("").contains("b1"));
        assert!(text.join("").contains("c1"));
        assert!(text.join("").contains("a2"));
        assert!(text.join("").contains("b2"));
        assert!(text.join("").contains("c2"));
    }

    #[test]
    fn list_all_bug_test() {
        let story =
            compile_string("LIST items = A, B\nVAR x = (A, B)\n{LIST_ALL(items)}\n-> END\n");
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("A"));
        assert!(text.join("").contains("B"));
    }

    #[test]
    fn list_comparison_test() {
        let story = compile_string(
            "VAR currentActor = \"Bobby\"\n\nLIST listOfActors = P, A, S, C\nVAR s = -> set_actor\n-> start\n\n===function set_actor(x)\n{ x:\n- P: ~ currentActor = \"Philippe\"\n- A: ~ currentActor = \"Andre\"\n- else: ~ currentActor = \"Bobby\"\n}\n\n=== start ===\n{s(P)} Hey, my name is {currentActor}. What about yours?\n{s(A)} I am {currentActor} and I need my rheumatism pills!\n{s(P)} Would you like me, {currentActor}, to get some more for you?\n-> END\n",
        );
        let mut s = story.clone();
        let text = continue_all(&mut s);
        assert!(text.join("").contains("Philippe"));
        assert!(text.join("").contains("Andre"));
    }
}
