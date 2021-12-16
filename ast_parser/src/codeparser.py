import re
import time


class RepeatType:
    No = 0
    Any = 1
    AtLeastOne = 2
    OneOrZero = 3
    Or = 4


class MatchType:
    No = 0
    Token = 1
    SubMatch = 2

    def __init__(self, kind, value="", target=None):
        self.kind = kind
        self.value = value
        self.target = target

    def __str__(self):
        s = "MatchType: "
        if self.kind == self.No:
            s += "No"
        if self.kind == self.Token:
            s += "Token[" + self.value + "]"
        if self.kind == self.SubMatch:
            s += "SubMatch"
            s += f"[{self.value}]"

        return s

    def __repr__(self):
        return self.__str__()

    def __hash__(self):
        return hash(self.kind) + hash(self.value)


class MatchTypeInstance:
    No = MatchType(MatchType.No)


def get_match_type(data):
    if not data:
        return MatchTypeInstance.No
    first = data[0]
    target = None
    if len(data) > 1:
        target = data[1]
    if first.startswith("Ast::"):
        return MatchType(MatchType.SubMatch, first[len("Ast::"):], target)
    else:
        return MatchType(MatchType.Token, first, target)


class CodeDefine:
    def __init__(self, regex):
        self.now_state = 0
        self.state_machine = {}
        self.regex = regex
        self.now_group = 0
        self.group_state = [()]
        self.group_state[0] = self.compile(regex, 0)

    def get_next_group(self):
        self.now_group += 1
        self.group_state.append(())
        return self.now_group

    def compile(self, regex, group):
        compiler = _CodeDefineCompile(regex, self, group)
        return compiler.start_state, compiler.now_end_state


class _CodeDefineCompile:
    def __init__(self, regex, regex_self, group):
        self.regex_self = regex_self
        self.regex = regex
        self.group = group
        self.now_pos = 0
        self.state = 0
        self.start_state = self.get_next_state()
        self.now_end_state = self.start_state

        self.compile()

    def match_repeat(self):
        if self.is_end():
            return RepeatType.No
        now = self.regex[self.now_pos]
        self.now_pos += 1
        if now == "*":
            return RepeatType.Any
        elif now == "?":
            return RepeatType.OneOrZero
        elif now == "+":
            return RepeatType.AtLeastOne
        elif now == "|":
            return RepeatType.Or
        self.now_pos -= 1
        return RepeatType.No

    def is_end(self):
        if len(self.regex) <= self.now_pos:
            return True
        return False

    def next_schema(self):
        if self.is_end():
            return None, None
        now = self.regex[self.now_pos]
        escape = False
        if now == "\\":
            escape = True
        elif now == "(":
            internal_escape = False
            left_count = 1
            pos = self.now_pos + 1
            res = -1
            for i in range(pos, len(self.regex)):
                now_item = self.regex[i]
                if internal_escape:
                    internal_escape = False
                    continue
                if now_item == "\\" and not internal_escape:
                    internal_escape = True
                    continue
                if now_item == "(":
                    left_count += 1
                    continue
                if now_item == ")":
                    left_count -= 1
                    if left_count == 0:
                        res = i
                        break
            if res == -1:
                raise Exception("group not match")
            group = self.regex_self.get_next_group()
            state_machine = self.regex_self.compile(self.regex[self.now_pos + 1: res], group)
            self.regex_self.group_state[group] = state_machine
            self.now_pos = res + 1
            # 无条件转移
            self.add_state_trans(self.get_now_state(), state_machine[0], "")

            return state_machine, self.match_repeat()

        data = now
        if escape:
            self.now_pos += 1
            if self.is_end():
                raise Exception("end in escape")
            now = self.regex[self.now_pos]
            data += now
        now_state = self.get_now_state()
        start_state = self.get_next_state()
        end_state = self.get_next_state()

        self.add_state_trans(now_state, start_state, "")

        self.add_state_trans(start_state, end_state, data)

        self.now_pos += 1
        return (start_state, end_state), self.match_repeat()

    def add_state_trans(self, from_state, to_state, data):
        data = get_match_type(data)
        if data not in self.regex_self.state_machine[from_state]:
            self.regex_self.state_machine[from_state][data] = set()
        self.regex_self.state_machine[from_state][data].add(to_state)

    def get_next_state(self):
        state = f"group{self.group}_state{self.state}"
        self.state += 1
        self.regex_self.state_machine[state] = {}
        return state

    def get_now_state(self):
        return self.now_end_state

    def copy_states(self, start):
        if start.startswith("copy_"):
            return start
        new_state_name = "copy_" + start
        trans = self.regex_self.state_machine[start]
        if new_state_name in self.regex_self.state_machine:
            return new_state_name
        self.regex_self.state_machine[new_state_name] = {}
        for k in trans:
            self.regex_self.state_machine[new_state_name][k] = set()
            for state in trans[k]:
                self.regex_self.state_machine[new_state_name][k].add(self.copy_states(state))
        return new_state_name

    def compile(self):
        while not self.is_end():
            state, schema = self.next_schema()
            self.handle_schema(state, schema)

    def handle_schema(self, state, schema):
        if not state:
            return
        start, end = state

        if schema == RepeatType.Or:
            new_end = self.get_next_state()
            self.add_state_trans(end, new_end, "")
            while True:
                state, schema = self.next_schema()
                if not state:
                    raise Exception("unknown or operator")
                # self.handle_schema(state, schema)
                if schema != RepeatType.No and schema != RepeatType.Or:
                    raise Exception("not support now, please wrap | item with ()")
                self.add_state_trans(state[1], new_end, "")
                if schema == RepeatType.No:
                    break
            self.now_end_state = new_end
            return

        self.now_end_state = end
        if schema == RepeatType.No:
            return
        if schema == RepeatType.AtLeastOne:
            new_start = self.copy_states(start)
            self.add_state_trans(end, new_start, "")
            start = new_start
            end = "copy_" + end
            self.now_end_state = end
        self.add_state_trans(start, end, "")
        if schema == RepeatType.OneOrZero:
            return
        self.add_state_trans(end, start, "")


class _NFAToDFA:
    def __init__(self, nfa, start, end):
        self.nfa = nfa
        self.nfa_start = start
        self.nfa_end = end
        self.state = 0
        self.state_machine = {}
        self.state_translate = {}
        self.state_translate_reverser = {}
        self.start_state = self.get_next_state()
        self.end_state = self.start_state
        self.eps_cache = {}

        self.visited = set()
        self.to_visit = []

        self.merged_state = {}

    def get_state(self, state_set):
        state = self.state_set_to_tuple(state_set)
        if state in self.state_translate_reverser:
            return self.state_translate_reverser[state]
        new_state = self.get_next_state()
        self.state_translate_reverser[state] = new_state
        self.state_translate[new_state] = state
        return new_state

    def get_next_state(self):
        state = f"state_{self.state}"
        self.state += 1
        self.state_machine[state] = {}
        return state

    def translate(self):
        start_state = self.find_eps_nfa(self.nfa_start)
        self.state_translate[self.start_state] = start_state
        self.state_translate_reverser[start_state] = self.start_state
        self.to_visit.append(self.start_state)

        while len(self.to_visit):
            state = self.to_visit.pop(0)
            self.find_all_trans_for_state(state)

    def find_eps_nfa(self, start, target=None):
        no_target = target is None
        if no_target:
            target = set()
        state = MatchTypeInstance.No
        if state not in self.nfa[start]:
            return start,
        target.add(start)
        to = self.nfa[start][state]
        for i in to:
            if i not in target:
                target.add(i)
                self.find_eps_nfa(i, target)
        res = self.state_set_to_tuple(target)
        if no_target:
            self.eps_cache[state] = res
        return res

    @staticmethod
    def state_set_to_tuple(state):
        return tuple(sorted(list(state)))

    def add_trans(self, from_state, to_state, condition):
        if condition in self.state_machine[from_state]:
            raise "Not DFA, only one trans allow"
        self.state_machine[from_state][condition] = to_state

    def find_all_trans_for_state(self, state):
        if state in self.visited:
            return
        self.visited.add(state)
        all_trans = {}
        for nfa_state in self.state_translate[state]:
            trans = self.nfa[nfa_state]
            for t in trans:
                if t in all_trans:
                    all_trans[t].update(trans[t])
                else:
                    all_trans[t] = set(trans[t])
        for t in all_trans:
            if len(all_trans[t]) == 0:
                continue
            if t == MatchTypeInstance.No:
                continue
            target = set()
            for to in all_trans[t]:
                target.add(to)
                target.update(self.find_eps_nfa(to))
            new_state = self.get_state(target)
            self.to_visit.append(new_state)
            self.add_trans(state, new_state, t)

    def is_end(self, state):
        nfa_states = self.state_translate[state]
        return self.nfa_end in nfa_states

    def find_merged_state(self, state):
        while state in self.merged_state:
            state = self.merged_state[state]
        return state

    def is_same_target(self, state1, state2):
        trans1 = self.state_machine[state1]
        trans2 = self.state_machine[state2]

        if len(trans1) != len(trans2):
            return False
        for cond in trans1:
            if cond not in trans2:
                return False
            target1 = trans1[cond]
            target2 = trans2[cond]
            for t1 in target1:
                for t2 in target2:
                    if self.find_merged_state(t1) == self.find_merged_state(t2):
                        break
                else:
                    return False
        return True

    def merge_state(self, state1, state2):
        small = state1
        big = state2
        if state1 > state2:
            small, big = state2, state1
        self.merged_state[big] = small

    def zip_one(self):
        change = False
        for state1 in self.state_machine:
            for state2 in self.state_machine:
                if self.find_merged_state(state1) == self.find_merged_state(state2):
                    continue
                if self.is_end(state1) != self.is_end(state2):
                    continue
                if not self.is_same_target(state1, state2):
                    continue
                change = True
                self.merge_state(state1, state2)
        return change

    def zip(self):
        while self.zip_one():
            pass

        class _GenStateMachine:
            def __init__(self, state_machine, start, merged):
                self.state_machine = state_machine
                self.start = start
                self.merged = merged
                self.new_state_machine = {}
                self.trans = {}
                self.state = 0
                self.visited = set()

            def get_next_state(self):
                state = self.state
                self.state += 1
                self.new_state_machine[state] = {}
                return state

            def get_state(self, old):
                if old in self.trans:
                    return self.trans[old]
                new_state = self.get_next_state()
                self.trans[old] = new_state
                return new_state

            def find_merged_state(self, state):
                while state in self.merged:
                    state = self.merged[state]
                return state

            def add_trans(self, from_state, to_state, cond):
                if cond not in self.new_state_machine[from_state]:
                    self.new_state_machine[from_state][cond] = set()
                self.new_state_machine[from_state][cond].add(to_state)

            def reduce(self, start):
                if start in self.visited:
                    return
                new_start = self.get_state(self.find_merged_state(start))
                self.visited.add(start)
                states = self.state_machine[start]
                for cond in states:
                    # if cond.kind != MatchType.SpecificChar:
                    #     raise Exception("not support")
                    i = states[cond]
                    root = self.find_merged_state(i)
                    self.new_state_machine[new_start][cond] = self.get_state(root)
                    self.reduce(i)

        g = _GenStateMachine(self.state_machine, self.start_state, self.merged_state)
        g.reduce(self.start_state)

        state = g.new_state_machine

        end_states = set()
        for i in g.trans:
            v = g.trans[i]
            if self.is_end(i):
                end_states.add(v)

        return state, end_states


class DFA:
    def __init__(self, state, start, end_states):
        self.state = state
        self.ends = end_states
        self.start = start

    def match(self, string):
        now_state = self.start
        for c in string:
            if c not in self.state[now_state]:
                return False
            now_state = self.state[now_state][c]
        return now_state in self.ends

    def generate(self, struct_name, is_enum=False):
        def add_ident(code, ident=1):
            lines = code.split("\n")
            return "\n".join(["    " * ident + line for line in lines if line]) + "\n"

        def init_state():
            return f"let now = stream.now_at;\nlet mut res = {struct_name}::default();\nlet mut state = 0;"

        def generate_to_next():
            return "stream.now_at += 1;"

        def generate_is_end():
            return "stream.is_end()"

        def generate_get_now():
            return "let top = &stream.tokens[stream.now_at];"

        def generate_ref(items):
            if len(items) == 0:
                raise Exception("Error items")
            if len(items) == 1:
                return items[0]
            if len(items) == 2:
                if items[1] == "_":
                    return f"{items[0]}(_)"
                return f"{items[0]}({items[0]}::{items[1]})"
            return f"{items[0]}({items[0]}::{generate_ref(items[1:])})"

        def generate_match(target):
            if target.kind == MatchType.Token:
                val = target.value
                items = val.split("::")
                ref = generate_ref(items)
                return f"let TokenKind::{ref} = top.kind"
            else:
                return f"let Ok(item) = {target.value}::parse(stream)"

        def generate_state(state):
            code = f"{state} => {{\n"
            buf = ""
            if len(self.state[state]) == 0:
                buf += "break"
            for cond in self.state[state]:
                buf += "{\n"
                if cond.kind == MatchType.Token:
                    buf += generate_get_now() + "\n"
                buf += f"if {generate_match(cond)} {{\n"
                buf += add_ident(f"state = {self.state[state][cond]};")
                if cond.kind == MatchType.Token:
                    buf += add_ident(generate_to_next())
                if is_enum:
                    if not cond.target:
                        cond.target = cond.value
                if cond.target:
                    t = "item"
                    if cond.kind == MatchType.Token:
                        t = "top.clone()"
                    if is_enum:
                        buf += add_ident(f"res = {struct_name}::{cond.target}({t});")
                    elif cond.target[0] == '.':
                        target = cond.target[1:]
                        buf += add_ident(f"res.{target}.push({t});")
                    else:
                        buf += add_ident(f"res.{cond.target} = {t};")
                buf += add_ident("continue")
                buf += "}\n"
                buf += "}\n"
            code += add_ident(buf)
            if len(self.state[state]) != 0:
                code += add_ident(generate_get_now())
                code += add_ident(f"stream.now_at = now;\nreturn Err(format!(\"Unexpected Token, got {{:?}} now: {state} \", top.kind))")
            code += "}\n"
            return code

        def generate_is_match():
            return "return Ok(res)"

        func = f"fn parse(stream: &mut TokenStream) -> Result<{struct_name}, String> {{\n"
        func += add_ident(init_state())
        func += add_ident(f"while !{generate_is_end()}{{")
        func += add_ident("match state {", 2)
        for i in self.state:
            func += add_ident(generate_state(i), 3)
        func += add_ident("_ => {", 3)
        func += add_ident('stream.now_at = now;\nreturn Err(String::from("unknown state"));', 4)
        func += add_ident("}\n", 3)
        func += add_ident("}\n", 2)
        func += add_ident("}\n")
        func += add_ident(generate_is_match())
        func += "}\n"
        trait = f"impl Parser for {struct_name} {{\n"
        trait += add_ident(f"type Output = {struct_name};")
        trait += add_ident(func)
        trait += "}\n"
        return trait


def regex(reg):
    r = CodeDefine(parse_code_defines(reg))
    dfa = _NFAToDFA(r.state_machine, r.group_state[0][0], r.group_state[0][1])
    dfa.translate()
    zipped, ends = dfa.zip()
    return DFA(zipped, 0, ends)


def is_bound(c):
    return is_space(c) or c in ['?', '*', '+', '(', ')', '|']


def is_space(c):
    return c in [' ', '\t', '\n', '\r']


def parse_code_defines(defines):
    defines = defines.replace("->", " -> ")

    tokens = []
    now = ""
    for i in defines:
        if not is_bound(i):
            now += i
        else:
            if now:
                tokens.append(now)
                now = ""
            if not is_space(i):
                tokens.append(i)
    if now:
        tokens.append(now)
    real_tokens = []
    now_item = None
    to_merge = False
    for i in tokens:
        if is_bound(i):
            if now_item:
                real_tokens.append((now_item,))
                now_item = None
            real_tokens.append(i)
            continue
        if to_merge:
            if not now_item:
                raise Exception("error status, now item == 0")
            if i == "->":
                raise Exception("error status")
            real_tokens.append((now_item, i))
            now_item = None
            to_merge = False
            continue
        if i == "->":
            to_merge = True
            continue
        else:
            if now_item:
                real_tokens.append((now_item,))
            now_item = i
    if now_item:
        real_tokens.append((now_item,))

    return tuple(real_tokens)


def parse_and_gen(reg, struct_name, is_enum = False):
    r = regex(reg)
    return r.generate(struct_name, is_enum)
