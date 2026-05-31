package com.lumirum.lumirumapp

import android.app.Application
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

class LumiRumApp : Application() {

    private val applicationScope = CoroutineScope(SupervisorJob() + Dispatchers.Main)

    val container by lazy { AppContainer(this) }

    override fun onCreate() {
        super.onCreate()
        applicationScope.launch {
            container.initialize()
        }
    }
}
