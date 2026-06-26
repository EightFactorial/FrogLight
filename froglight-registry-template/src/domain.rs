//! A custom [`hazarc::Domain`](Domain) with support for `no-std` environments.
//!
//! Requires `no-std` to provide and initialize the [`RegistryDomain`] at
//! runtime.

use hazarc::domain::{Domain, DomainList, DomainNodeRef};

use crate::types::OnceLock;

cfg_select! {
    feature = "std" => {
        /// A type alias for the default [`hazarc::Domain`](Domain) implementation.
        pub type DefaultDomain = hazarc::DefaultDomain;
    }
    _ => {
        /// A type alias for the default [`hazarc::Domain`](Domain) implementation.
        pub type DefaultDomain = RegistryDomain;
    }
}

/// A custom [`hazarc::Domain`](Domain) implementation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryDomain;

static DOMAIN: OnceLock<RegistryDomainFns> = OnceLock::new();

impl RegistryDomain {
    /// Initialize the [`RegistryDomain`] with the given [`CustomDomain`].
    ///
    /// Returns `true` if the [`RegistryDomain`] was initialized,
    /// or `false` if it was already initialized.
    pub fn try_init<T: CustomDomain>() -> bool { DOMAIN.set(RegistryDomainFns::new::<T>()).is_ok() }

    /// Get the [`RegistryDomainFns`] for the current [`RegistryDomain`].å
    ///
    /// # Panics
    ///
    /// Panics if the [`RegistryDomain`] has not been initialized.
    #[must_use]
    fn domain() -> &'static RegistryDomainFns {
        DOMAIN.get().expect("RegistryDomain not initialized!")
    }

    // ---------------------------------------------------------------------------------------------

    /// See [`Domain::static_list`].
    fn static_list() -> &'static DomainList<RegistryDomain> { (Self::domain().static_list_fn)() }

    /// See [`Domain::set_thread_local_node`].
    unsafe fn set_thread_local_node(node: Option<DomainNodeRef<RegistryDomain>>) {
        unsafe { (Self::domain().set_thread_local_node_fn)(node) }
    }

    /// See [`Domain::get_thread_local_node`].
    fn get_thread_local_node() -> Option<DomainNodeRef<RegistryDomain>> {
        (Self::domain().get_thread_local_node_fn)()
    }

    /// See [`Domain::get_or_acquire_thread_local_node`].
    fn get_or_acquire_thread_local_node() -> DomainNodeRef<RegistryDomain> {
        (Self::domain().get_or_acquire_thread_local_node_fn)()
    }

    /// See [`Domain::release_thread_local_node`].
    fn release_thread_local_node() { (Self::domain().release_thread_local_node_fn)() }
}

unsafe impl Domain for RegistryDomain {
    const BORROW_SLOT_COUNT: usize = 8;

    #[inline]
    fn static_list() -> &'static DomainList<Self> { Self::static_list() }

    #[inline]
    fn get_thread_local_node() -> Option<DomainNodeRef<Self>> { Self::get_thread_local_node() }

    #[inline]
    unsafe fn set_thread_local_node(node: Option<DomainNodeRef<Self>>) {
        unsafe { Self::set_thread_local_node(node) }
    }

    #[inline]
    fn get_or_acquire_thread_local_node() -> DomainNodeRef<Self> {
        Self::get_or_acquire_thread_local_node()
    }

    #[inline]
    fn release_thread_local_node() { Self::release_thread_local_node() }
}

// -------------------------------------------------------------------------------------------------

#[allow(clippy::struct_field_names, reason = "Function Pointers")]
struct RegistryDomainFns {
    static_list_fn: fn() -> &'static DomainList<RegistryDomain>,
    get_thread_local_node_fn: fn() -> Option<DomainNodeRef<RegistryDomain>>,
    set_thread_local_node_fn: unsafe fn(Option<DomainNodeRef<RegistryDomain>>),
    get_or_acquire_thread_local_node_fn: fn() -> DomainNodeRef<RegistryDomain>,
    release_thread_local_node_fn: fn(),
}

impl RegistryDomainFns {
    /// Create a new [`RegistryDomainFns`]
    #[inline]
    #[must_use]
    const fn new<T: CustomDomain>() -> Self {
        Self {
            static_list_fn: <T as CustomDomain>::static_list,
            get_thread_local_node_fn: <T as CustomDomain>::get_thread_local_node,
            set_thread_local_node_fn: <T as CustomDomain>::set_thread_local_node,
            get_or_acquire_thread_local_node_fn:
                <T as CustomDomain>::get_or_acquire_thread_local_node,
            release_thread_local_node_fn: <T as CustomDomain>::release_thread_local_node,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper trait over [`hazarc::Domain`](Domain) that allows for
/// [`RegistryDomain`] to be initialized with a custom domain implementation.
///
/// See [`Domain`] for more information.
#[expect(clippy::missing_safety_doc, reason = "See `Domain`")]
pub unsafe trait CustomDomain: Domain {
    /// See [`Domain::static_list`].
    fn static_list() -> &'static DomainList<RegistryDomain>;
    /// See [`Domain::get_thread_local_node`].
    fn get_thread_local_node() -> Option<DomainNodeRef<RegistryDomain>>;
    /// See [`Domain::set_thread_local_node`].
    #[expect(clippy::missing_safety_doc, reason = "See `Domain`")]
    unsafe fn set_thread_local_node(node: Option<DomainNodeRef<RegistryDomain>>);
    /// See [`Domain::get_or_acquire_thread_local_node`].
    fn get_or_acquire_thread_local_node() -> DomainNodeRef<RegistryDomain>;
    /// See [`Domain::release_thread_local_node`].
    fn release_thread_local_node();
}
