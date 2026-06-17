/// https://developer.android.com/reference/android/net/wifi/WifiManager
///
/// API level 1
jni::bind_java_type! {
    pub AndroidWifiManager => "android.net.wifi.WifiManager",
    type_map = {
        AndroidWifiLock => "android.net.wifi.WifiManager$WifiLock",
        AndroidWifiMulticastLock => "android.net.wifi.WifiManager$MulticastLock",
    },
    methods {
        /// https://developer.android.com/reference/android/net/wifi/WifiManager#createWifiLock(int,%20java.lang.String)
        ///
        /// API level 3
        pub fn create_wifi_lock(lock_type: jint, tag: JString) -> AndroidWifiLock,

        /// https://developer.android.com/reference/android/net/wifi/WifiManager#createMulticastLock(java.lang.String)
        ///
        /// API level 4
        pub fn create_multicast_lock(tag: JString) -> AndroidWifiMulticastLock,
    }
}

/// https://developer.android.com/reference/android/net/wifi/WifiManager.WifiLock
///
/// API level 1
jni::bind_java_type! {
    pub AndroidWifiLock => "android.net.wifi.WifiManager$WifiLock",
    methods {
        /// https://developer.android.com/reference/android/net/wifi/WifiManager.WifiLock#acquire()
        ///
        /// API level 1
        pub fn acquire(),

        /// https://developer.android.com/reference/android/net/wifi/WifiManager.WifiLock#isHeld()
        ///
        /// API level 1
        pub fn is_held() -> bool,

        /// https://developer.android.com/reference/android/net/wifi/WifiManager.WifiLock#release()
        ///
        /// API level 1
        pub fn release(),
    }
}

/// https://developer.android.com/reference/android/net/wifi/WifiManager.MulticastLock
///
/// API level 4
jni::bind_java_type! {
    pub AndroidWifiMulticastLock => "android.net.wifi.WifiManager$MulticastLock",
    methods {
        /// https://developer.android.com/reference/android/net/wifi/WifiManager.MulticastLock#acquire()
        ///
        /// API level 4
        pub fn acquire(),

        /// https://developer.android.com/reference/android/net/wifi/WifiManager.MulticastLock#isHeld()
        ///
        /// API level 4
        pub fn is_held() -> bool,

        /// https://developer.android.com/reference/android/net/wifi/WifiManager.MulticastLock#release()
        ///
        /// API level 4
        pub fn release(),
    }
}
