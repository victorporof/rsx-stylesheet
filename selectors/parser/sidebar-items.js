initSidebarItems({"enum":[["Combinator",""],["Component","A CSS simple selector or combinator. We store both in the same enum for optimal packing and cache performance, see [1]."],["SelectorParseErrorKind",""]],"fn":[["is_css2_pseudo_element","Returns whether the name corresponds to a CSS2 pseudo-element that can be specified with the single colon syntax (in addition to the double-colon syntax, which can be used for all pseudo-elements)."],["namespace_empty_string",""],["parse_compound_selector_list","Parse a comma separated list of compound selectors."]],"struct":[["AncestorHashes","Ancestor hashes for the bloom filter. We precompute these and store them inline with selectors to optimize cache performance during matching. This matters a lot."],["LocalName",""],["Selector","A Selector stores a sequence of simple selectors and combinators. The iterator classes allow callers to iterate at either the raw sequence level or at the level of sequences of simple selectors separated by combinators. Most callers want the higher-level iterator."],["SelectorIter",""],["SelectorList",""]],"trait":[["Parser",""],["PseudoElement","A trait that represents a pseudo-element."],["SelectorImpl","This trait allows to define the parser implementation in regards of pseudo-classes/elements"],["SelectorMethods",""]],"type":[["SelectorParseError",""]]});