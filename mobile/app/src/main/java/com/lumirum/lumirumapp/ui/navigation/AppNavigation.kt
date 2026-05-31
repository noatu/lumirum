package com.lumirum.lumirumapp.ui.navigation

import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DevicesOther
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.SupervisorAccount
import androidx.compose.material.icons.filled.Tune
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.navigation.*
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.lumirum.lumirumapp.LocalAppContainer
import com.lumirum.lumirumapp.R
import com.lumirum.lumirumapp.data.api.dto.Role
import com.lumirum.lumirumapp.ui.screen.account.AccountScreen
import com.lumirum.lumirumapp.ui.screen.admin.AdminScreen
import com.lumirum.lumirumapp.ui.screen.devices.DeviceDetailScreen
import com.lumirum.lumirumapp.ui.screen.devices.DevicesScreen
import com.lumirum.lumirumapp.ui.screen.login.LoginScreen
import com.lumirum.lumirumapp.ui.screen.profiles.ProfileDetailScreen
import com.lumirum.lumirumapp.ui.screen.profiles.ProfilesScreen
import com.lumirum.lumirumapp.ui.screen.register.RegisterScreen
import com.lumirum.lumirumapp.ui.screen.schedule.LightingScheduleScreen
import com.lumirum.lumirumapp.ui.screen.telemetry.TelemetryScreen

sealed class BottomNavItem(val route: String, val labelRes: Int, val icon: androidx.compose.ui.graphics.vector.ImageVector) {
    data object Devices : BottomNavItem("devices", R.string.devices, Icons.Default.DevicesOther)
    data object Profiles : BottomNavItem("profiles", R.string.profiles, Icons.Default.Tune)
    data object Admin : BottomNavItem("admin", R.string.admin, Icons.Default.SupervisorAccount)
    data object Account : BottomNavItem("account", R.string.account, Icons.Default.Person)
}

private val baseNavItems = listOf(BottomNavItem.Devices, BottomNavItem.Profiles, BottomNavItem.Account)
private val adminNavItems = listOf(BottomNavItem.Devices, BottomNavItem.Profiles, BottomNavItem.Admin, BottomNavItem.Account)

@Composable
fun AppNavigation() {
    val container = LocalAppContainer.current
    val navController = rememberNavController()
    var isInitialized by remember { mutableStateOf(false) }
    var startDestination by remember { mutableStateOf("login") }
    val userRole by container.userRole.collectAsState()

    LaunchedEffect(Unit) {
        val token = container.dataStore.getToken()
        container.token = token
        startDestination = if (token != null) "devices" else "login"
        isInitialized = true
    }

    LaunchedEffect(Unit) {
        container.unauthorizedEvent.collect {
            container.logout()
            navController.navigate("login") {
                popUpTo(0) { inclusive = true }
            }
        }
    }

    if (!isInitialized) {
        Surface { com.lumirum.lumirumapp.ui.components.LoadingView() }
        return
    }

    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentRoute = navBackStackEntry?.destination?.route
    val navItems = if (userRole is Role.Admin) adminNavItems else baseNavItems
    val bottomNavRoutes = navItems.map { it.route }.toSet()
    val showBottomBar = currentRoute in bottomNavRoutes

    Scaffold(
        bottomBar = {
            if (showBottomBar) {
                NavigationBar {
                    navItems.forEach { item ->
                        NavigationBarItem(
                            selected = currentRoute == item.route,
                            onClick = {
                                navController.navigate(item.route) {
                                    popUpTo(navController.graph.findStartDestination().id) {
                                        saveState = true
                                    }
                                    launchSingleTop = true
                                    restoreState = true
                                }
                            },
                            icon = { Icon(item.icon, contentDescription = stringResource(item.labelRes)) },
                            label = { Text(stringResource(item.labelRes)) },
                        )
                    }
                }
            }
        },
    ) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = startDestination,
            modifier = Modifier.padding(innerPadding),
        ) {
            composable("login") {
                LoginScreen(
                    onLoginSuccess = {
                        navController.navigate("devices") {
                            popUpTo("login") { inclusive = true }
                        }
                    },
                    onRegister = { navController.navigate("register") },
                )
            }
            composable("register") {
                RegisterScreen(
                    onSuccess = {
                        navController.navigate("devices") {
                            popUpTo(0) { inclusive = true }
                        }
                    },
                    onBack = { navController.popBackStack() },
                )
            }

            composable("devices") {
                DevicesScreen(
                    onDeviceClick = { id -> navController.navigate("device/$id") },
                    onCreateClick = { navController.navigate("device-create") },
                )
            }
            composable("device-create") {
                DeviceDetailScreen(
                    deviceId = null,
                    onBack = { navController.popBackStack() },
                    onViewTelemetry = {},
                )
            }
            composable("device/{id}", arguments = listOf(navArgument("id") { type = NavType.LongType })) { entry ->
                val id = entry.arguments!!.getLong("id")
                DeviceDetailScreen(
                    deviceId = id,
                    onBack = { navController.popBackStack() },
                    onViewTelemetry = { navController.navigate("telemetry/$id") },
                )
            }
            composable("telemetry/{id}", arguments = listOf(navArgument("id") { type = NavType.LongType })) { entry ->
                val id = entry.arguments!!.getLong("id")
                TelemetryScreen(deviceId = id, onBack = { navController.popBackStack() })
            }

            composable("profiles") {
                ProfilesScreen(
                    onProfileClick = { id -> navController.navigate("profile/$id") },
                    onCreateClick = { navController.navigate("profile-create") },
                )
            }
            composable("profile-create") {
                ProfileDetailScreen(profileId = null, onBack = { navController.popBackStack() })
            }
            composable("profile/{id}", arguments = listOf(navArgument("id") { type = NavType.LongType })) { entry ->
                val id = entry.arguments!!.getLong("id")
                ProfileDetailScreen(
                    profileId = id,
                    onBack = { navController.popBackStack() },
                    onViewSchedule = { navController.navigate("schedule/$id") },
                )
            }
            composable("schedule/{id}", arguments = listOf(navArgument("id") { type = NavType.LongType })) { entry ->
                val id = entry.arguments!!.getLong("id")
                LightingScheduleScreen(profileId = id, onBack = { navController.popBackStack() })
            }

            composable("account") {
                AccountScreen(onLoggedOut = {
                    navController.navigate("login") {
                        popUpTo(0) { inclusive = true }
                    }
                })
            }

            composable("admin") {
                AdminScreen()
            }
        }
    }
}
