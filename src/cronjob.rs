#![allow(non_snake_case)]

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// CronJobSpec defines the desired state of CronJob
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "batch.tutorial.kubebuilder.io", version = "v2", kind = "CronJob", plural = "cronjobs")]
#[kube(namespaced)]
#[kube(status = "CronJobStatus")]
#[kube(schema = "disabled")]
pub struct CronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are: - "Allow" (default): allows CronJobs to run concurrently. - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet. - "Replace": cancels currently running job and replaces it with a new one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrencyPolicy: Option<String>,
    /// The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failedJobHistoryLimit: Option<i32>,
    /// Specifies the job that will be created when executing a job.
    pub jobTemplate: CronJobJobTemplate,
    /// The schedule in Cron format,see https://en.wikipedia.org/wiki/Cron.
    pub schedule: CronJobSchedule,
    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason. Missed jobs executions will be counted as failed ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startingDeadlineSeconds: Option<i64>,
    /// The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successfulJobHistoryLimit: Option<i32>,
    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}

/// Specifies the job that will be created when executing a job.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplate {
    /// Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CronJobJobTemplateMetadata>,
    /// Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<CronJobJobTemplateSpec>,
}

/// Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateMetadata {
}

/// Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpec {
    /// Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activeDeadlineSeconds: Option<i64>,
    /// Specifies the number of retries before marking this job failed. Defaults to 6
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoffLimit: Option<i32>,
    /// CompletionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`. 
    ///  `NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other. 
    ///  `Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`. 
    ///  This field is beta-level. More completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, the controller skips updates for the Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completionMode: Option<String>,
    /// Specifies the desired number of successfully finished pods the job should be run with.  Setting to nil means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    /// manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manualSelector: Option<bool>,
    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    /// A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CronJobJobTemplateSpecSelector>,
    /// Suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to true, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from false to true), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the StartTime field of the Job, effectively resetting the ActiveDeadlineSeconds timer too. Defaults to false. 
    ///  This field is beta-level, gated by SuspendJob feature flag (enabled by default).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Describes the pod that will be created when executing a job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    pub template: CronJobJobTemplateSpecTemplate,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttlSecondsAfterFinished: Option<i32>,
}

/// A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Describes the pod that will be created when executing a job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplate {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CronJobJobTemplateSpecTemplateMetadata>,
    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<CronJobJobTemplateSpecTemplateSpec>,
}

/// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateMetadata {
}

/// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpec {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activeDeadlineSeconds: Option<i64>,
    /// If specified, the pod's scheduling constraints
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<CronJobJobTemplateSpecTemplateSpecAffinity>,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automountServiceAccountToken: Option<bool>,
    /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
    pub containers: Vec<CronJobJobTemplateSpecTemplateSpecContainers>,
    /// Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dnsConfig: Option<CronJobJobTemplateSpecTemplateSpecDnsConfig>,
    /// Set DNS policy for the pod. Defaults to "ClusterFirst". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dnsPolicy: Option<String>,
    /// EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enableServiceLinks: Option<bool>,
    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource. This field is beta-level and available on clusters that haven't disabled the EphemeralContainers feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeralContainers: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainers>>,
    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-hostNetwork pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostAliases: Option<Vec<CronJobJobTemplateSpecTemplateSpecHostAliases>>,
    /// Use the host's ipc namespace. Optional: Default to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostIPC: Option<bool>,
    /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostNetwork: Option<bool>,
    /// Use the host's pid namespace. Optional: Default to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostPID: Option<bool>,
    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imagePullSecrets: Option<Vec<CronJobJobTemplateSpecTemplateSpecImagePullSecrets>>,
    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initContainers: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainers>>,
    /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodeName: Option<String>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodeSelector: Option<BTreeMap<String, String>>,
    /// Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set. 
    ///  If the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions 
    ///  If the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.containers[*].securityContext.seLinuxOptions - spec.containers[*].securityContext.seccompProfile - spec.containers[*].securityContext.capabilities - spec.containers[*].securityContext.readOnlyRootFilesystem - spec.containers[*].securityContext.privileged - spec.containers[*].securityContext.allowPrivilegeEscalation - spec.containers[*].securityContext.procMount - spec.containers[*].securityContext.runAsUser - spec.containers[*].securityContext.runAsGroup This is an alpha field and requires the IdentifyPodOS feature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<CronJobJobTemplateSpecTemplateSpecOs>,
    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md This field is beta-level as of Kubernetes v1.18, and is only honored by servers that enable the PodOverhead feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<BTreeMap<String, IntOrString>>,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. This field is beta-level, gated by the NonPreemptingPriority feature-gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preemptionPolicy: Option<String>,
    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priorityClassName: Option<String>,
    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readinessGates: Option<Vec<CronJobJobTemplateSpecTemplateSpecReadinessGates>>,
    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restartPolicy: Option<String>,
    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class This is a beta feature as of Kubernetes v1.14.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtimeClassName: Option<String>,
    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedulerName: Option<String>,
    /// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub securityContext: Option<CronJobJobTemplateSpecTemplateSpecSecurityContext>,
    /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serviceAccount: Option<String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serviceAccountName: Option<String>,
    /// If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setHostnameAsFQDN: Option<bool>,
    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shareProcessNamespace: Option<bool>,
    /// If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>". If not specified, the pod will not have a domainname at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<CronJobJobTemplateSpecTemplateSpecTolerations>>,
    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topologySpreadConstraints: Option<Vec<CronJobJobTemplateSpecTemplateSpecTopologySpreadConstraints>>,
    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumes>>,
}

/// If specified, the pod's scheduling constraints
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinity {
    /// Describes node affinity scheduling rules for the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodeAffinity: Option<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinity>,
    /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub podAffinity: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinity>,
    /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub podAntiAffinity: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinity>,
}

/// Describes node affinity scheduling rules for the pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferredDuringSchedulingIgnoredDuringExecution: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requiredDuringSchedulingIgnoredDuringExecution: Option<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

/// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    /// A node selector term, associated with the corresponding weight.
    pub preference: CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference,
    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    pub weight: i32,
}

/// A node selector term, associated with the corresponding weight.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchFields: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    /// Required. A list of node selector terms. The terms are ORed.
    pub nodeSelectorTerms: Vec<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms>,
}

/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchFields: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferredDuringSchedulingIgnoredDuringExecution: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requiredDuringSchedulingIgnoredDuringExecution: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    /// Required. A pod affinity term, associated with the corresponding weight.
    pub podAffinityTerm: CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    pub weight: i32,
}

/// Required. A pod affinity term, associated with the corresponding weight.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labelSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaceSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    pub topologyKey: String,
}

/// A label query over a set of resources, in this case pods.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    /// A label query over a set of resources, in this case pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labelSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaceSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    pub topologyKey: String,
}

/// A label query over a set of resources, in this case pods.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinity {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferredDuringSchedulingIgnoredDuringExecution: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requiredDuringSchedulingIgnoredDuringExecution: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    /// Required. A pod affinity term, associated with the corresponding weight.
    pub podAffinityTerm: CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    pub weight: i32,
}

/// Required. A pod affinity term, associated with the corresponding weight.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    /// A label query over a set of resources, in this case pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labelSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaceSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    pub topologyKey: String,
}

/// A label query over a set of resources, in this case pods.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    /// A label query over a set of resources, in this case pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labelSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaceSelector: Option<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    pub topologyKey: String,
}

/// A label query over a set of resources, in this case pods.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A single application container that you want to run within a pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainers {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// List of environment variables to set in the container. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersEnv>>,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envFrom: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersEnvFrom>>,
    /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imagePullPolicy: Option<String>,
    /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecycle>,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub livenessProbe: Option<CronJobJobTemplateSpecTemplateSpecContainersLivenessProbe>,
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
    pub name: String,
    /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersPorts>>,
    /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readinessProbe: Option<CronJobJobTemplateSpecTemplateSpecContainersReadinessProbe>,
    /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CronJobJobTemplateSpecTemplateSpecContainersResources>,
    /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub securityContext: Option<CronJobJobTemplateSpecTemplateSpecContainersSecurityContext>,
    /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startupProbe: Option<CronJobJobTemplateSpecTemplateSpecContainersStartupProbe>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdinOnce: Option<bool>,
    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationMessagePath: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationMessagePolicy: Option<String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeDevices: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersVolumeDevices>>,
    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeMounts: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersVolumeMounts>>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workingDir: Option<String>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valueFrom: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMapKeyRef: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldRef: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceFieldRef: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretKeyRef: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiVersion: Option<String>,
    /// Path of the field to select in the specified API version.
    pub fieldPath: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containerName: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvFrom {
    /// The ConfigMap to select from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMapRef: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvFromConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The Secret to select from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecContainersEnvFromSecretRef>,
}

/// The ConfigMap to select from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvFromConfigMapRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The Secret to select from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersEnvFromSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postStart: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStart>,
    /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preStop: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStop>,
}

/// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStart {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartExec>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartHttpGet>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartTcpSocket>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePostStartTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStop {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopExec>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopHttpGet>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopTcpSocket>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLifecyclePreStopTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLivenessProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersLivenessProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersPorts {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
    pub containerPort: i32,
    /// What host IP to bind the external port to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostIP: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostPort: Option<i32>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersReadinessProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersReadinessProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersSecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowPrivilegeEscalation: Option<bool>,
    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CronJobJobTemplateSpecTemplateSpecContainersSecurityContextCapabilities>,
    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procMount: Option<String>,
    /// Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnlyRootFilesystem: Option<bool>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsGroup: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsNonRoot: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUser: Option<i64>,
    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seLinuxOptions: Option<CronJobJobTemplateSpecTemplateSpecContainersSecurityContextSeLinuxOptions>,
    /// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccompProfile: Option<CronJobJobTemplateSpecTemplateSpecContainersSecurityContextSeccompProfile>,
    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windowsOptions: Option<CronJobJobTemplateSpecTemplateSpecContainersSecurityContextWindowsOptions>,
}

/// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersSecurityContextCapabilities {
    /// Added capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// Removed capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersSecurityContextSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersSecurityContextSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhostProfile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are: 
    ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    pub r#type: String,
}

/// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersSecurityContextWindowsOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpec: Option<String>,
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpecName: Option<String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostProcess: Option<bool>,
    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUserName: Option<String>,
}

/// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersStartupProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecContainersStartupProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecContainersStartupProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecContainersStartupProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecContainersStartupProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersStartupProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersStartupProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersStartupProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecContainersStartupProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersStartupProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersStartupProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersVolumeDevices {
    /// devicePath is the path inside of the container that the device will be mapped to.
    pub devicePath: String,
    /// name must match the name of a persistentVolumeClaim in the pod
    pub name: String,
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecContainersVolumeMounts {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mountPath: String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mountPropagation: Option<String>,
    /// This must match the Name of a Volume.
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPath: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPathExpr: Option<String>,
}

/// Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecDnsConfig {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CronJobJobTemplateSpecTemplateSpecDnsConfigOptions>>,
    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searches: Option<Vec<String>>,
}

/// PodDNSConfigOption defines DNS resolver options of a pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecDnsConfigOptions {
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// An EphemeralContainer is a temporary container that you may add to an existing Pod for user-initiated activities such as debugging. Ephemeral containers have no resource or scheduling guarantees, and they will not be restarted when they exit or when a Pod is removed or restarted. The kubelet may evict a Pod if an ephemeral container causes the Pod to exceed its resource allocation. 
///  To add an ephemeral container, use the ephemeralcontainers subresource of an existing Pod. Ephemeral containers may not be removed or restarted. 
///  This is a beta feature available on clusters that haven't disabled the EphemeralContainers feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainers {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// List of environment variables to set in the container. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnv>>,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envFrom: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvFrom>>,
    /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imagePullPolicy: Option<String>,
    /// Lifecycle is not allowed for ephemeral containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecycle>,
    /// Probes are not allowed for ephemeral containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub livenessProbe: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbe>,
    /// Name of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.
    pub name: String,
    /// Ports are not allowed for ephemeral containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersPorts>>,
    /// Probes are not allowed for ephemeral containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readinessProbe: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbe>,
    /// Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersResources>,
    /// Optional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub securityContext: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContext>,
    /// Probes are not allowed for ephemeral containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startupProbe: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbe>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdinOnce: Option<bool>,
    /// If set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container uses the namespaces configured in the Pod spec. 
    ///  The container runtime must implement support for this feature. If the runtime does not support namespace targeting then the result of setting this field is undefined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targetContainerName: Option<String>,
    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationMessagePath: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationMessagePolicy: Option<String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeDevices: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersVolumeDevices>>,
    /// Pod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeMounts: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersVolumeMounts>>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workingDir: Option<String>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valueFrom: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMapKeyRef: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldRef: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceFieldRef: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretKeyRef: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiVersion: Option<String>,
    /// Path of the field to select in the specified API version.
    pub fieldPath: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containerName: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvFrom {
    /// The ConfigMap to select from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMapRef: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvFromConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The Secret to select from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvFromSecretRef>,
}

/// The ConfigMap to select from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvFromConfigMapRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The Secret to select from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersEnvFromSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Lifecycle is not allowed for ephemeral containers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postStart: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStart>,
    /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preStop: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStop>,
}

/// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStart {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartExec>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartHttpGet>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartTcpSocket>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePostStartTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStop {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopExec>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopHttpGet>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopTcpSocket>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLifecyclePreStopTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// Probes are not allowed for ephemeral containers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersLivenessProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersPorts {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
    pub containerPort: i32,
    /// What host IP to bind the external port to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostIP: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostPort: Option<i32>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// Probes are not allowed for ephemeral containers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersReadinessProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// Optional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowPrivilegeEscalation: Option<bool>,
    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextCapabilities>,
    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procMount: Option<String>,
    /// Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnlyRootFilesystem: Option<bool>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsGroup: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsNonRoot: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUser: Option<i64>,
    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seLinuxOptions: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextSeLinuxOptions>,
    /// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccompProfile: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextSeccompProfile>,
    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windowsOptions: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextWindowsOptions>,
}

/// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextCapabilities {
    /// Added capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// Removed capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhostProfile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are: 
    ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    pub r#type: String,
}

/// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersSecurityContextWindowsOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpec: Option<String>,
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpecName: Option<String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostProcess: Option<bool>,
    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUserName: Option<String>,
}

/// Probes are not allowed for ephemeral containers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersStartupProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersVolumeDevices {
    /// devicePath is the path inside of the container that the device will be mapped to.
    pub devicePath: String,
    /// name must match the name of a persistentVolumeClaim in the pod
    pub name: String,
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecEphemeralContainersVolumeMounts {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mountPath: String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mountPropagation: Option<String>,
    /// This must match the Name of a Volume.
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPath: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPathExpr: Option<String>,
}

/// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecHostAliases {
    /// Hostnames for the above IP address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// IP address of the host file entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecImagePullSecrets {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A single application container that you want to run within a pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainers {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// List of environment variables to set in the container. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersEnv>>,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envFrom: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersEnvFrom>>,
    /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imagePullPolicy: Option<String>,
    /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecycle>,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub livenessProbe: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbe>,
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
    pub name: String,
    /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersPorts>>,
    /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readinessProbe: Option<CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbe>,
    /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CronJobJobTemplateSpecTemplateSpecInitContainersResources>,
    /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub securityContext: Option<CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContext>,
    /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startupProbe: Option<CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbe>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdinOnce: Option<bool>,
    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationMessagePath: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationMessagePolicy: Option<String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeDevices: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersVolumeDevices>>,
    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeMounts: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersVolumeMounts>>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workingDir: Option<String>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valueFrom: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMapKeyRef: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldRef: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceFieldRef: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretKeyRef: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiVersion: Option<String>,
    /// Path of the field to select in the specified API version.
    pub fieldPath: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containerName: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvFrom {
    /// The ConfigMap to select from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMapRef: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvFromConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The Secret to select from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecInitContainersEnvFromSecretRef>,
}

/// The ConfigMap to select from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvFromConfigMapRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The Secret to select from
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersEnvFromSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecycle {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postStart: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStart>,
    /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preStop: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStop>,
}

/// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStart {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartExec>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartHttpGet>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartTcpSocket>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePostStartTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStop {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopExec>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopHttpGet>,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopTcpSocket>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLifecyclePreStopTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersLivenessProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersPorts {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
    pub containerPort: i32,
    /// What host IP to bind the external port to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostIP: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostPort: Option<i32>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersReadinessProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContext {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowPrivilegeEscalation: Option<bool>,
    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextCapabilities>,
    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub procMount: Option<String>,
    /// Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnlyRootFilesystem: Option<bool>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsGroup: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsNonRoot: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUser: Option<i64>,
    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seLinuxOptions: Option<CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextSeLinuxOptions>,
    /// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccompProfile: Option<CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextSeccompProfile>,
    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windowsOptions: Option<CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextWindowsOptions>,
}

/// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextCapabilities {
    /// Added capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// Removed capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhostProfile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are: 
    ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    pub r#type: String,
}

/// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersSecurityContextWindowsOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpec: Option<String>,
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpecName: Option<String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostProcess: Option<bool>,
    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUserName: Option<String>,
}

/// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbe {
    /// Exec specifies the action to take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failureThreshold: Option<i32>,
    /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeGrpc>,
    /// HTTPGet specifies the http request to perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpGet: Option<CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialDelaySeconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodSeconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successThreshold: Option<i32>,
    /// TCPSocket specifies an action involving a TCP port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcpSocket: Option<CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeTcpSocket>,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminationGracePeriodSeconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoutSeconds: Option<i32>,
}

/// Exec specifies the action to take.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeGrpc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    pub port: i32,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
    ///  If this is not specified, the default behavior is defined by gRPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

/// HTTPGet specifies the http request to perform.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub httpHeaders: Option<Vec<CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeHttpGetHttpHeaders {
    /// The header field name
    pub name: String,
    /// The header field value
    pub value: String,
}

/// TCPSocket specifies an action involving a TCP port.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersStartupProbeTcpSocket {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    pub port: IntOrString,
}

/// volumeDevice describes a mapping of a raw block device within a container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersVolumeDevices {
    /// devicePath is the path inside of the container that the device will be mapped to.
    pub devicePath: String,
    /// name must match the name of a persistentVolumeClaim in the pod
    pub name: String,
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecInitContainersVolumeMounts {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mountPath: String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mountPropagation: Option<String>,
    /// This must match the Name of a Volume.
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPath: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subPathExpr: Option<String>,
}

/// Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set. 
///  If the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions 
///  If the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.containers[*].securityContext.seLinuxOptions - spec.containers[*].securityContext.seccompProfile - spec.containers[*].securityContext.capabilities - spec.containers[*].securityContext.readOnlyRootFilesystem - spec.containers[*].securityContext.privileged - spec.containers[*].securityContext.allowPrivilegeEscalation - spec.containers[*].securityContext.procMount - spec.containers[*].securityContext.runAsUser - spec.containers[*].securityContext.runAsGroup This is an alpha field and requires the IdentifyPodOS feature
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecOs {
    /// Name is the name of the operating system. The currently supported values are linux and windows. Additional value may be defined in future and can be one of: https://github.com/opencontainers/runtime-spec/blob/master/config.md#platform-specific-configuration Clients should expect to handle additional values and treat unrecognized values in this field as os: null
    pub name: String,
}

/// PodReadinessGate contains the reference to a pod condition
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecReadinessGates {
    /// ConditionType refers to a condition in the pod's condition list with matching type.
    pub conditionType: String,
}

/// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecSecurityContext {
    /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod: 
    ///  1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw---- 
    ///  If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsGroup: Option<i64>,
    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsGroupChangePolicy: Option<String>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsGroup: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsNonRoot: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUser: Option<i64>,
    /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seLinuxOptions: Option<CronJobJobTemplateSpecTemplateSpecSecurityContextSeLinuxOptions>,
    /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccompProfile: Option<CronJobJobTemplateSpecTemplateSpecSecurityContextSeccompProfile>,
    /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplementalGroups: Option<Vec<i64>>,
    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Vec<CronJobJobTemplateSpecTemplateSpecSecurityContextSysctls>>,
    /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windowsOptions: Option<CronJobJobTemplateSpecTemplateSpecSecurityContextWindowsOptions>,
}

/// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecSecurityContextSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecSecurityContextSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localhostProfile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are: 
    ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    pub r#type: String,
}

/// Sysctl defines a kernel parameter to be set
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecSecurityContextSysctls {
    /// Name of a property to set
    pub name: String,
    /// Value of a property to set
    pub value: String,
}

/// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecSecurityContextWindowsOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpec: Option<String>,
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gmsaCredentialSpecName: Option<String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostProcess: Option<bool>,
    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runAsUserName: Option<String>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerationSeconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecTopologySpreadConstraints {
    /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labelSelector: Option<CronJobJobTemplateSpecTemplateSpecTopologySpreadConstraintsLabelSelector>,
    /// MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 1/1/0: | zone1 | zone2 | zone3 | |   P   |   P   |       | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 1/1/1; scheduling it onto zone1(zone2) would make the ActualSkew(2-0) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
    pub maxSkew: i32,
    /// TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a "bucket", and try to put balanced number of pods into each bucket. It's a required field.
    pub topologyKey: String,
    /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location, but giving higher precedence to topologies that would help reduce the skew. A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
    pub whenUnsatisfiable: String,
}

/// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecTopologySpreadConstraintsLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecTopologySpreadConstraintsLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecTopologySpreadConstraintsLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumes {
    /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awsElasticBlockStore: Option<CronJobJobTemplateSpecTemplateSpecVolumesAwsElasticBlockStore>,
    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azureDisk: Option<CronJobJobTemplateSpecTemplateSpecVolumesAzureDisk>,
    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azureFile: Option<CronJobJobTemplateSpecTemplateSpecVolumesAzureFile>,
    /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<CronJobJobTemplateSpecTemplateSpecVolumesCephfs>,
    /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cinder: Option<CronJobJobTemplateSpecTemplateSpecVolumesCinder>,
    /// ConfigMap represents a configMap that should populate this volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMap: Option<CronJobJobTemplateSpecTemplateSpecVolumesConfigMap>,
    /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csi: Option<CronJobJobTemplateSpecTemplateSpecVolumesCsi>,
    /// DownwardAPI represents downward API about the pod that should populate this volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downwardAPI: Option<CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPI>,
    /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emptyDir: Option<CronJobJobTemplateSpecTemplateSpecVolumesEmptyDir>,
    /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
    ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
    ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
    ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
    ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeral>,
    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fc: Option<CronJobJobTemplateSpecTemplateSpecVolumesFc>,
    /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexVolume: Option<CronJobJobTemplateSpecTemplateSpecVolumesFlexVolume>,
    /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flocker: Option<CronJobJobTemplateSpecTemplateSpecVolumesFlocker>,
    /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcePersistentDisk: Option<CronJobJobTemplateSpecTemplateSpecVolumesGcePersistentDisk>,
    /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gitRepo: Option<CronJobJobTemplateSpecTemplateSpecVolumesGitRepo>,
    /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<CronJobJobTemplateSpecTemplateSpecVolumesGlusterfs>,
    /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostPath: Option<CronJobJobTemplateSpecTemplateSpecVolumesHostPath>,
    /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<CronJobJobTemplateSpecTemplateSpecVolumesIscsi>,
    /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
    /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfs: Option<CronJobJobTemplateSpecTemplateSpecVolumesNfs>,
    /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistentVolumeClaim: Option<CronJobJobTemplateSpecTemplateSpecVolumesPersistentVolumeClaim>,
    /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photonPersistentDisk: Option<CronJobJobTemplateSpecTemplateSpecVolumesPhotonPersistentDisk>,
    /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portworxVolume: Option<CronJobJobTemplateSpecTemplateSpecVolumesPortworxVolume>,
    /// Items for all in one resources secrets, configmaps, and downward API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projected: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjected>,
    /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<CronJobJobTemplateSpecTemplateSpecVolumesQuobyte>,
    /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rbd: Option<CronJobJobTemplateSpecTemplateSpecVolumesRbd>,
    /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaleIO: Option<CronJobJobTemplateSpecTemplateSpecVolumesScaleIO>,
    /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<CronJobJobTemplateSpecTemplateSpecVolumesSecret>,
    /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storageos: Option<CronJobJobTemplateSpecTemplateSpecVolumesStorageos>,
    /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsphereVolume: Option<CronJobJobTemplateSpecTemplateSpecVolumesVsphereVolume>,
}

/// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesAwsElasticBlockStore {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore TODO: how do we prevent errors in the filesystem from compromising the machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If omitted, the default is "false". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    pub volumeID: String,
}

/// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesAzureDisk {
    /// Host Caching mode: None, Read Only, Read Write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cachingMode: Option<String>,
    /// The Name of the data disk in the blob storage
    pub diskName: String,
    /// The URI the data disk in the blob storage
    pub diskURI: String,
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
}

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesAzureFile {
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// the name of secret that contains Azure Storage Account Name and Key
    pub secretName: String,
    /// Share Name
    pub shareName: String,
}

/// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesCephfs {
    /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    pub monitors: Vec<String>,
    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretFile: Option<String>,
    /// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesCephfsSecretRef>,
    /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesCephfsSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesCinder {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Optional: points to a secret object containing parameters used to connect to OpenStack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesCinderSecretRef>,
    /// volume id used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    pub volumeID: String,
}

/// Optional: points to a secret object containing parameters used to connect to OpenStack.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesCinderSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ConfigMap represents a configMap that should populate this volume
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesConfigMap {
    /// Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaultMode: Option<i32>,
    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesConfigMapItems>>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesConfigMapItems {
    /// The key to project.
    pub key: String,
    /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}

/// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesCsi {
    /// Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
    pub driver: String,
    /// Filesystem type to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodePublishSecretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesCsiNodePublishSecretRef>,
    /// Specifies a read-only configuration for the volume. Defaults to false (read/write).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeAttributes: Option<BTreeMap<String, String>>,
}

/// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesCsiNodePublishSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DownwardAPI represents downward API about the pod that should populate this volume
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPI {
    /// Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaultMode: Option<i32>,
    /// Items is a list of downward API volume file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPIItems>>,
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPIItems {
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPIItemsFieldRef>,
    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: String,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceFieldRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPIItemsResourceFieldRef>,
}

/// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPIItemsFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiVersion: Option<String>,
    /// Path of the field to select in the specified API version.
    pub fieldPath: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesDownwardAPIItemsResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containerName: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEmptyDir {
    /// What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sizeLimit: Option<IntOrString>,
}

/// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeral {
    /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
    ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
    ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
    ///  Required, must not be nil.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeClaimTemplate: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplate>,
}

/// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
///  Required, must not be nil.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplate {
    /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateMetadata>,
    /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
    pub spec: CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpec,
}

/// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateMetadata {
}

/// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpec {
    /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessModes: Option<Vec<String>>,
    /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataSource: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecDataSource>,
    /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataSourceRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecDataSourceRef>,
    /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecResources>,
    /// A label query over volumes to consider for binding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecSelector>,
    /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storageClassName: Option<String>,
    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeMode: Option<String>,
    /// VolumeName is the binding reference to the PersistentVolume backing this claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeName: Option<String>,
}

/// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecDataSource {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiGroup: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecDataSourceRef {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiGroup: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// A label query over volumes to consider for binding.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchExpressions: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchLabels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesEphemeralVolumeClaimTemplateSpecSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesFc {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. TODO: how do we prevent errors in the filesystem from compromising the machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Optional: FC target lun number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Optional: FC target worldwide names (WWNs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targetWWNs: Option<Vec<String>>,
    /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wwids: Option<Vec<String>>,
}

/// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesFlexVolume {
    /// Driver is the name of the driver to use for this volume.
    pub driver: String,
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Optional: Extra command options if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesFlexVolumeSecretRef>,
}

/// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesFlexVolumeSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesFlocker {
    /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasetName: Option<String>,
    /// UUID of the dataset. This is unique identifier of a Flocker dataset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasetUUID: Option<String>,
}

/// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesGcePersistentDisk {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk TODO: how do we prevent errors in the filesystem from compromising the machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    pub pdName: String,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
}

/// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesGitRepo {
    /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// Repository URL
    pub repository: String,
    /// Commit hash for the specified revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

/// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesGlusterfs {
    /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    pub endpoints: String,
    /// Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    pub path: String,
    /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
}

/// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesHostPath {
    /// Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    pub path: String,
    /// Type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesIscsi {
    /// whether support iSCSI Discovery CHAP authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chapAuthDiscovery: Option<bool>,
    /// whether support iSCSI Session CHAP authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chapAuthSession: Option<bool>,
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi TODO: how do we prevent errors in the filesystem from compromising the machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiatorName: Option<String>,
    /// Target iSCSI Qualified Name.
    pub iqn: String,
    /// iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iscsiInterface: Option<String>,
    /// iSCSI Target Lun number.
    pub lun: i32,
    /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portals: Option<Vec<String>>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// CHAP Secret for iSCSI target and initiator authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesIscsiSecretRef>,
    /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    pub targetPortal: String,
}

/// CHAP Secret for iSCSI target and initiator authentication
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesIscsiSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesNfs {
    /// Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub path: String,
    /// ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    pub server: String,
}

/// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesPersistentVolumeClaim {
    /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    pub claimName: String,
    /// Will force the ReadOnly setting in VolumeMounts. Default false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
}

/// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesPhotonPersistentDisk {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// ID that identifies Photon Controller persistent disk
    pub pdID: String,
}

/// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesPortworxVolume {
    /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// VolumeID uniquely identifies a Portworx volume
    pub volumeID: String,
}

/// Items for all in one resources secrets, configmaps, and downward API
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjected {
    /// Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaultMode: Option<i32>,
    /// list of volume projections
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSources>>,
}

/// Projection that may be projected along with other supported volume types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSources {
    /// information about the configMap data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configMap: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesConfigMap>,
    /// information about the downwardAPI data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downwardAPI: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPI>,
    /// information about the secret data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesSecret>,
    /// information about the serviceAccountToken data to project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serviceAccountToken: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesServiceAccountToken>,
}

/// information about the configMap data to project
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesConfigMap {
    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesConfigMapItems>>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesConfigMapItems {
    /// The key to project.
    pub key: String,
    /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}

/// information about the downwardAPI data to project
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPI {
    /// Items is a list of DownwardAPIVolume file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPIItems>>,
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPIItems {
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPIItemsFieldRef>,
    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: String,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceFieldRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPIItemsResourceFieldRef>,
}

/// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPIItemsFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiVersion: Option<String>,
    /// Path of the field to select in the specified API version.
    pub fieldPath: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesDownwardAPIItemsResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containerName: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// information about the secret data to project
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesSecret {
    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesSecretItems>>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesSecretItems {
    /// The key to project.
    pub key: String,
    /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}

/// information about the serviceAccountToken data to project
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesProjectedSourcesServiceAccountToken {
    /// Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expirationSeconds: Option<i64>,
    /// Path is the path relative to the mount point of the file to project the token into.
    pub path: String,
}

/// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesQuobyte {
    /// Group to map volume access to Default is no group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
    pub registry: String,
    /// Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    /// User to map volume access to Defaults to serivceaccount user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Volume is a string that references an already created Quobyte volume by name.
    pub volume: String,
}

/// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesRbd {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd TODO: how do we prevent errors in the filesystem from compromising the machine
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// The rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub image: String,
    /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyring: Option<String>,
    /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    pub monitors: Vec<String>,
    /// The rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesRbdSecretRef>,
    /// The rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesRbdSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesScaleIO {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// The host address of the ScaleIO API Gateway.
    pub gateway: String,
    /// The name of the ScaleIO Protection Domain for the configured storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protectionDomain: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
    pub secretRef: CronJobJobTemplateSpecTemplateSpecVolumesScaleIOSecretRef,
    /// Flag to enable/disable SSL communication with Gateway, default false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sslEnabled: Option<bool>,
    /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storageMode: Option<String>,
    /// The ScaleIO Storage Pool associated with the protection domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storagePool: Option<String>,
    /// The name of the storage system as configured in ScaleIO.
    pub system: String,
    /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeName: Option<String>,
}

/// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesScaleIOSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesSecret {
    /// Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaultMode: Option<i32>,
    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CronJobJobTemplateSpecTemplateSpecVolumesSecretItems>>,
    /// Specify whether the Secret or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretName: Option<String>,
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesSecretItems {
    /// The key to project.
    pub key: String,
    /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
    pub path: String,
}

/// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesStorageos {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readOnly: Option<bool>,
    /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secretRef: Option<CronJobJobTemplateSpecTemplateSpecVolumesStorageosSecretRef>,
    /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeName: Option<String>,
    /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumeNamespace: Option<String>,
}

/// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesStorageosSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobJobTemplateSpecTemplateSpecVolumesVsphereVolume {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fsType: Option<String>,
    /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storagePolicyID: Option<String>,
    /// Storage Policy Based Management (SPBM) profile name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storagePolicyName: Option<String>,
    /// Path that identifies vSphere volume vmdk
    pub volumePath: String,
}

/// The schedule in Cron format,see https://en.wikipedia.org/wiki/Cron.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobSchedule {
    /// specifies the day of the month during which the job executes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dayOfMonth: Option<String>,
    /// specifies the day of the week during which the job executes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dayOfWeek: Option<String>,
    /// specifies the hour during which the job executes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    /// specifies the minute during which the job executes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<String>,
    /// specifies the month during which the job executes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
}

/// CronJobStatus defines the observed state of CronJob
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<CronJobStatusActive>>,
    /// Information when was the last time the job was successfully scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastScheduleTime: Option<String>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object. --- New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs. 1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage. 2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted". Those cannot be well described when embedded. 3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen. 4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple and the version of the actual struct is irrelevant. 5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control. Instead of using this type, create a locally provided and used type that is well-focused on your reference. For example, ServiceReferences for admission registration: https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533 .
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronJobStatusActive {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiVersion: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldPath: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourceVersion: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

