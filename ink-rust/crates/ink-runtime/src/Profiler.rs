// Source: ink-c-sharp/ink-engine-runtime/Profiler.cs

use crate::CallStack::CallStack;
use crate::Container::ContentItem;
use crate::ControlCommand::ControlCommand;
use crate::Path::Path;
use crate::Value::Value;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Debug, Default)]
pub struct Profiler {
    continue_watch: Option<Instant>,
    step_watch: Option<Instant>,
    snap_watch: Option<Instant>,
    continue_total: f64,
    snap_total: f64,
    step_total: f64,
    curr_step_stack: Option<Vec<String>>,
    curr_step_details: StepDetails,
    root_node: ProfileNode,
    num_continues: i32,
    step_details: Vec<StepDetails>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StepDetails {
    pub r#type: String,
    pub obj: Option<ContentItem>,
    pub path: String,
    pub time: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProfileNode {
    pub key: String,
    pub openInUI: bool,
    nodes: HashMap<String, ProfileNode>,
    total_millisecs: f64,
    self_millisecs: f64,
    self_sample_count: i32,
    total_sample_count: i32,
}

impl Profiler {
    // C# signature: public Profiler()
    pub fn new() -> Self {
        Self {
            root_node: ProfileNode::new(),
            ..Default::default()
        }
    }

    // C# signature: public string Report()
    pub fn Report(&self) -> String {
        let mut sb = String::new();
        sb.push_str(&format!("{} CONTINUES / LINES:\n", self.num_continues));
        sb.push_str(&format!(
            "TOTAL TIME: {}\n",
            Self::FormatMillisecs(self.continue_total)
        ));
        sb.push_str(&format!(
            "SNAPSHOTTING: {}\n",
            Self::FormatMillisecs(self.snap_total)
        ));
        sb.push_str(&format!(
            "OTHER: {}\n",
            Self::FormatMillisecs(self.continue_total - (self.step_total + self.snap_total))
        ));
        sb.push_str(&self.root_node.ToString());
        sb
    }

    // C# signature: public void PreContinue()
    pub fn PreContinue(&mut self) {
        self.continue_watch = Some(Instant::now());
    }

    // C# signature: public void PostContinue()
    pub fn PostContinue(&mut self) {
        if let Some(start) = self.continue_watch.take() {
            self.continue_total += elapsed_millis(start);
            self.num_continues += 1;
        }
    }

    // C# signature: public void PreStep()
    pub fn PreStep(&mut self) {
        self.curr_step_stack = None;
        self.step_watch = Some(Instant::now());
    }

    // C# signature: public void Step(CallStack callstack)
    pub fn Step(&mut self, callstack: &CallStack) {
        self.step_watch = None;

        let callstack_elements = callstack.get_callStack();
        let mut stack = Vec::with_capacity(callstack_elements.len());

        for element in &callstack_elements {
            let mut stack_element_name = String::new();
            if !element.currentPointer.get_isNull() {
                if let Some(obj_path) = element.currentPointer.get_path() {
                    for c in 0..obj_path.get_length() {
                        if let Some(comp) = obj_path.GetComponent(c) {
                            if !comp.get_isIndex() {
                                stack_element_name = comp.get_name().unwrap_or("").to_string();
                                break;
                            }
                        }
                    }
                }
            }
            stack.push(stack_element_name);
        }

        self.curr_step_stack = Some(stack);

        let curr_obj = callstack.currentElement().currentPointer.Resolve();
        let current_path = callstack
            .currentElement()
            .currentPointer
            .get_path()
            .map(|path| path.ToString())
            .unwrap_or_default();

        let step_type = match curr_obj.as_ref() {
            Some(ContentItem::ControlCommand(control_command)) => {
                format!("{:?} CC", control_command.commandType)
            }
            Some(content_item) => content_item_type_name(content_item),
            None => String::new(),
        };

        self.curr_step_details = StepDetails {
            r#type: step_type,
            obj: curr_obj,
            path: current_path,
            time: 0.0,
        };

        self.step_watch = Some(Instant::now());
    }

    // C# signature: public void PostStep()
    pub fn PostStep(&mut self) {
        if let Some(start) = self.step_watch.take() {
            let duration = elapsed_millis(start);
            self.step_total += duration;

            if let Some(stack) = self.curr_step_stack.as_ref() {
                self.root_node.AddSample(stack.clone(), duration);
            }

            self.curr_step_details.time = duration;
            self.step_details.push(self.curr_step_details.clone());
        }
    }

    // C# signature: public string StepLengthReport()
    pub fn StepLengthReport(&self) -> String {
        let mut sb = String::new();
        sb.push_str(&format!(
            "TOTAL: {}ms\n",
            self.root_node.get_totalMillisecs()
        ));

        let mut average_step_times = group_step_details(&self.step_details)
            .into_iter()
            .map(|(kind, details)| (kind, average_duration(details)))
            .collect::<Vec<_>>();
        average_step_times
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let average_step_times = average_step_times
            .into_iter()
            .map(|(kind, time)| format!("{kind}: {time}ms"))
            .collect::<Vec<_>>();
        sb.push_str(&format!(
            "AVERAGE STEP TIMES: {}\n",
            average_step_times.join(", ")
        ));

        let mut accum_step_times = group_step_details(&self.step_details)
            .into_iter()
            .map(|(kind, details)| {
                (
                    format!("{kind} (x{})", details.len()),
                    sum_duration(details),
                )
            })
            .collect::<Vec<_>>();
        accum_step_times.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let accum_step_times = accum_step_times
            .into_iter()
            .map(|(kind, time)| format!("{kind}: {time}"))
            .collect::<Vec<_>>();
        sb.push_str(&format!(
            "ACCUMULATED STEP TIMES: {}\n",
            accum_step_times.join(", ")
        ));

        sb
    }

    // C# signature: public string Megalog()
    pub fn Megalog(&self) -> String {
        let mut sb = String::new();
        sb.push_str("Step type\tDescription\tPath\tTime\n");

        for step in &self.step_details {
            sb.push_str(&step.r#type);
            sb.push('\t');
            sb.push_str(&content_item_to_description(step.obj.as_ref()));
            sb.push('\t');
            sb.push_str(&step.path);
            sb.push('\t');
            sb.push_str(&format!("{:.8}", step.time));
            sb.push('\n');
        }

        sb
    }

    // C# signature: public void PreSnapshot()
    pub fn PreSnapshot(&mut self) {
        self.snap_watch = Some(Instant::now());
    }

    // C# signature: public void PostSnapshot()
    pub fn PostSnapshot(&mut self) {
        if let Some(start) = self.snap_watch.take() {
            self.snap_total += elapsed_millis(start);
        }
    }

    // C# signature: public static string FormatMillisecs(double num)
    pub fn FormatMillisecs(num: f64) -> String {
        if num > 5000.0 {
            format!("{} secs", format_grouped_number(num / 1000.0, 1))
        } else if num > 1000.0 {
            format!("{} secs", format_grouped_number(num / 1000.0, 2))
        } else if num > 100.0 {
            format!("{} ms", format_grouped_number(num, 0))
        } else if num > 1.0 {
            format!("{} ms", format_grouped_number(num, 1))
        } else if num > 0.01 {
            format!("{} ms", format_grouped_number(num, 3))
        } else {
            format!("{} ms", format_grouped_number(num, 2))
        }
    }

    // C# signature: public void AddSample(string[] stack, double duration)
    pub fn AddSample(&mut self, stack: Vec<String>, duration: f64) {
        self.root_node.AddSample(stack, duration);
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        let mut sb = String::new();
        self.root_node.PrintHierarchy(&mut sb, 0);
        sb
    }

    // C# signature: ProfileNode rootNode { get; }
    pub fn get_rootNode(&self) -> &ProfileNode {
        &self.root_node
    }

    // C# signature: bool hasChildren { get; }
    pub fn get_hasChildren(&self) -> bool {
        self.root_node.get_hasChildren()
    }

    // C# signature: int totalMillisecs { get; }
    pub fn get_totalMillisecs(&self) -> i32 {
        self.root_node.get_totalMillisecs()
    }

    // C# signature: string ownReport { get; }
    pub fn get_ownReport(&self) -> String {
        self.root_node.get_ownReport()
    }
}

impl ProfileNode {
    // C# signature: public ProfileNode()
    pub fn new() -> Self {
        Self {
            key: String::new(),
            ..Default::default()
        }
    }

    // C# signature: public ProfileNode(string key)
    pub fn new_overload_2(key: String) -> Self {
        Self {
            key,
            ..Default::default()
        }
    }

    // C# signature: public void AddSample(string[] stack, double duration)
    pub fn AddSample(&mut self, stack: Vec<String>, duration: f64) {
        self.add_sample(&stack, -1, duration);
    }

    fn add_sample(&mut self, stack: &[String], stack_idx: i32, duration: f64) {
        self.total_sample_count += 1;
        self.total_millisecs += duration;

        if stack_idx == stack.len() as i32 - 1 {
            self.self_sample_count += 1;
            self.self_millisecs += duration;
        }

        if stack_idx + 1 < stack.len() as i32 {
            self.add_sample_to_node(stack, stack_idx + 1, duration);
        }
    }

    fn add_sample_to_node(&mut self, stack: &[String], stack_idx: i32, duration: f64) {
        let node_key = stack[stack_idx as usize].clone();
        let node = self
            .nodes
            .entry(node_key.clone())
            .or_insert_with(|| ProfileNode::new_overload_2(node_key));
        node.add_sample(stack, stack_idx, duration);
    }

    // C# signature: IEnumerable<KeyValuePair<string, ProfileNode>> descendingOrderedNodes { get; }
    pub fn descendingOrderedNodes(&self) -> Option<Vec<(String, ProfileNode)>> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut nodes = self
            .nodes
            .iter()
            .map(|(key, node)| (key.clone(), node.clone()))
            .collect::<Vec<_>>();
        nodes.sort_by(|a, b| {
            b.1.total_millisecs
                .partial_cmp(&a.1.total_millisecs)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Some(nodes)
    }

    fn PrintHierarchy(&self, sb: &mut String, indent: i32) {
        self.pad(sb, indent);
        sb.push_str(&self.key);
        sb.push_str(": ");
        sb.push_str(&self.ownReport());
        sb.push('\n');

        if let Some(nodes) = self.descendingOrderedNodes() {
            for (_, node) in nodes {
                node.PrintHierarchy(sb, indent + 1);
            }
        }
    }

    // C# signature: string ownReport { get; }
    pub fn ownReport(&self) -> String {
        let mut sb = String::new();
        sb.push_str("total ");
        sb.push_str(&Profiler::FormatMillisecs(self.total_millisecs));
        sb.push_str(", self ");
        sb.push_str(&Profiler::FormatMillisecs(self.self_millisecs));
        sb.push_str(" (");
        sb.push_str(&self.self_sample_count.to_string());
        sb.push_str(" self samples, ");
        sb.push_str(&self.total_sample_count.to_string());
        sb.push_str(" total)");
        sb
    }

    fn pad(&self, sb: &mut String, spaces: i32) {
        for _ in 0..spaces {
            sb.push_str("   ");
        }
    }

    // C# signature: public override string ToString ()
    pub fn ToString(&self) -> String {
        let mut sb = String::new();
        self.PrintHierarchy(&mut sb, 0);
        sb
    }

    // C# signature: bool hasChildren { get; }
    pub fn get_hasChildren(&self) -> bool {
        !self.nodes.is_empty()
    }

    // C# signature: int totalMillisecs { get; }
    pub fn get_totalMillisecs(&self) -> i32 {
        self.total_millisecs as i32
    }

    // C# signature: string ownReport { get; }
    pub fn get_ownReport(&self) -> String {
        self.ownReport()
    }
}

fn elapsed_millis(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn group_step_details<'a>(
    step_details: &'a [StepDetails],
) -> HashMap<String, Vec<&'a StepDetails>> {
    let mut grouped: HashMap<String, Vec<&'a StepDetails>> = HashMap::new();
    for step in step_details {
        grouped.entry(step.r#type.clone()).or_default().push(step);
    }
    grouped
}

fn average_duration(details: Vec<&StepDetails>) -> f64 {
    if details.is_empty() {
        0.0
    } else {
        details.iter().map(|step| step.time).sum::<f64>() / details.len() as f64
    }
}

fn sum_duration(details: Vec<&StepDetails>) -> f64 {
    details.iter().map(|step| step.time).sum::<f64>()
}

fn content_item_type_name(content_item: &ContentItem) -> String {
    match content_item {
        ContentItem::Value(value) => match value {
            Value::Bool(_) => "BoolValue".to_string(),
            Value::Int(_) => "IntValue".to_string(),
            Value::Float(_) => "FloatValue".to_string(),
            Value::String(_) => "StringValue".to_string(),
            Value::DivertTarget(_) => "DivertTargetValue".to_string(),
            Value::VariablePointer(_) => "VariablePointerValue".to_string(),
            Value::List(_) => "ListValue".to_string(),
        },
        ContentItem::ControlCommand(command) => format!("{:?} CC", command.commandType),
        ContentItem::Void(_) => "Void".to_string(),
        ContentItem::Container(_) => "Container".to_string(),
        ContentItem::VariableReference(_) => "VariableReference".to_string(),
        ContentItem::Divert(_) => "Divert".to_string(),
        ContentItem::ChoicePoint(_) => "ChoicePoint".to_string(),
        ContentItem::Glue(_) => "Glue".to_string(),
        ContentItem::NativeFunctionCall(_) => "NativeFunctionCall".to_string(),
        ContentItem::VariableAssignment(_) => "VariableAssignment".to_string(),
        ContentItem::Tag(_) => "Tag".to_string(),
        ContentItem::Choice(_) => "Choice".to_string(),
    }
}

fn content_item_to_description(content_item: Option<&ContentItem>) -> String {
    match content_item {
        Some(ContentItem::Value(value)) => value.ToString(),
        Some(ContentItem::ControlCommand(command)) => command.ToString(),
        Some(ContentItem::Void(_)) => "Void".to_string(),
        Some(ContentItem::Container(_)) => "Container".to_string(),
        Some(ContentItem::VariableReference(reference)) => reference.ToString(),
        Some(ContentItem::Divert(divert)) => divert.ToString(),
        Some(ContentItem::ChoicePoint(choice_point)) => choice_point.ToString(),
        Some(ContentItem::Glue(glue)) => glue.ToString(),
        Some(ContentItem::NativeFunctionCall(call)) => call.ToString(),
        Some(ContentItem::VariableAssignment(var)) => var.ToString(),
        Some(ContentItem::Tag(tag)) => tag.ToString(),
        Some(ContentItem::Choice(choice)) => choice.get_pathStringOnChoice().unwrap_or_default(),
        None => String::new(),
    }
}

fn format_grouped_number(num: f64, digits: usize) -> String {
    let raw = format!("{:.*}", digits, num);
    let (sign, unsigned) = if let Some(rest) = raw.strip_prefix('-') {
        ("-", rest)
    } else {
        ("", raw.as_str())
    };

    let mut parts = unsigned.splitn(2, '.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next();
    let mut grouped = String::new();

    for (idx, ch) in int_part.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(ch);
    }

    let int_part = grouped.chars().rev().collect::<String>();
    match frac_part {
        Some(frac) if digits > 0 => format!("{sign}{int_part}.{frac}"),
        _ => format!("{sign}{int_part}"),
    }
}

#[cfg(test)]
mod tests {
    use super::Profiler;
    use crate::CallStack::{CallStack, Element, Thread};
    use crate::Container::{Container, ContentItem};
    use crate::ControlCommand::ControlCommand;
    use crate::Path::Path;
    use crate::Pointer::Pointer;
    use crate::PushPop::PushPopType;
    use std::rc::Rc;

    fn callstack_with_command(command: ControlCommand) -> CallStack {
        let mut container = Container::new();
        container.AddContent(command);

        let pointer = Pointer::StartOf(Rc::new(container));
        let element = Element::new(PushPopType::Tunnel, pointer, false);
        let thread = Thread {
            callstack: vec![element],
            threadIndex: 0,
            previousPointer: None,
        };

        CallStack {
            threads: vec![thread],
            threadCounter: 0,
            startOfRoot: Pointer::Null(),
        }
    }

    #[test]
    fn formats_millisecs_like_csharp_thresholds() {
        assert_eq!(Profiler::FormatMillisecs(6000.0), "6.0 secs");
        assert_eq!(Profiler::FormatMillisecs(1500.0), "1.50 secs");
        assert_eq!(Profiler::FormatMillisecs(120.0), "120 ms");
        assert_eq!(Profiler::FormatMillisecs(12.3), "12.3 ms");
        assert_eq!(Profiler::FormatMillisecs(0.234), "0.234 ms");
        assert_eq!(Profiler::FormatMillisecs(0.004), "0.00 ms");
    }

    #[test]
    fn records_step_and_report_text() {
        let mut profiler = Profiler::new();
        let callstack = callstack_with_command(ControlCommand::BeginString());

        profiler.PreContinue();
        profiler.PostContinue();
        profiler.PreStep();
        profiler.Step(&callstack);
        profiler.PostStep();
        profiler.PreSnapshot();
        profiler.PostSnapshot();

        let report = profiler.Report();
        assert!(report.contains("CONTINUES / LINES"));
        assert!(report.contains("TOTAL TIME"));
        assert!(report.contains("SNAPSHOTTING"));

        let step_report = profiler.StepLengthReport();
        assert!(step_report.contains("TOTAL:"));
        assert!(step_report.contains("AVERAGE STEP TIMES:"));
        assert!(step_report.contains("ACCUMULATED STEP TIMES:"));

        let megalog = profiler.Megalog();
        assert!(megalog.contains("Step type\tDescription\tPath\tTime"));
        assert!(megalog.contains("BeginString"));
    }

    #[test]
    fn hierarchy_accumulates_samples() {
        let mut node = super::ProfileNode::new();
        node.AddSample(vec!["root".to_string(), "child".to_string()], 2.0);
        node.AddSample(vec!["root".to_string(), "child".to_string()], 1.0);
        node.AddSample(vec!["root".to_string(), "other".to_string()], 3.0);

        assert!(node.get_hasChildren());
        assert_eq!(node.get_totalMillisecs(), 6);
        assert!(node.get_ownReport().contains("total 6.0 ms"));
        assert!(node.ToString().contains("root"));
        assert!(node.ToString().contains("child"));
    }
}
