package com.lumirum.lumirumapp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.CompositionLocalProvider
import com.lumirum.lumirumapp.ui.navigation.AppNavigation
import com.lumirum.lumirumapp.ui.theme.LumiRumTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val container = (application as LumiRumApp).container
        setContent {
            LumiRumTheme {
                CompositionLocalProvider(LocalAppContainer provides container) {
                    AppNavigation()
                }
            }
        }
    }
}
