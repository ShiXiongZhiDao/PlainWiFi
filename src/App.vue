<template>
  <div class="password-manager font-brut" :class="{ 'dark': isDarkMode }">
    <div class="flex h-screen bg-brut-white dark:bg-brut-dark-bg">
      <!-- Left sidebar - Password list -->
      <div
          class="flex flex-col relative bg-brut-yellow border-r-4 border-black dark:bg-brut-dark-card dark:border-brut-white"
          :style="{ width: `${sidebarWidth}px` }"
      >
        <div class="p-6 flex-1 min-h-0 overflow-y-auto">
          <h1 class="text-2xl font-black uppercase tracking-tight text-black dark:text-white mb-6 border-b-4 border-black pb-2 dark:border-brut-white">{{ t('appTitle') }}</h1>
          <div class="relative mb-6">
            <input
                v-model="searchKey"
                type="text"
                :placeholder="t('searchPlaceholder')"
                class="w-full p-3 pl-10 bg-brut-white rounded-none border-[3px] border-black text-black font-bold placeholder:text-black/50 focus:outline-none focus:shadow-[4px_4px_0_#000] dark:bg-brut-dark-bg dark:text-white dark:border-brut-white dark:placeholder:text-white/50 dark:focus:shadow-[4px_4px_0_#fff] transition-shadow"
            />
            <span class="absolute left-3 top-3 text-black dark:text-brut-yellow">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-search"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
            </span>
            <!-- Clear search button -->
            <button
                v-if="searchKey"
                @click="clearSearch"
                class="absolute right-3 top-3 text-black font-black hover:text-brut-magenta dark:text-white dark:hover:text-brut-magenta transition-colors"
                :aria-label="t('clearSearchAria')"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
            </button>
          </div>

          <div class="password-list space-y-3">
            <div
                v-for="item in filteredPasswords"
                :key="item.id"
                @click="selectPassword(item)"
                class="p-4 rounded-none cursor-pointer border-[3px] border-black shadow-[3px_3px_0_#000] bg-brut-white dark:bg-brut-dark-card dark:border-brut-white dark:shadow-[3px_3px_0_#fff] hover:translate-x-[-2px] hover:translate-y-[-2px] hover:shadow-[5px_5px_0_#000] dark:hover:shadow-[5px_5px_0_#fff] active:translate-x-[2px] active:translate-y-[2px] active:shadow-none transition-all"
                :class="{
                'bg-brut-magenta text-black shadow-[3px_3px_0_#000] dark:bg-brut-magenta dark:text-black': selectedPassword && selectedPassword.id === item.id,
              }"
            >
              <div class="flex items-center min-w-0">
                <div class="mr-3 shrink-0 text-black border-2 border-black p-2 rounded-none bg-brut-electric dark:bg-brut-electric dark:text-black" :class="{'bg-brut-yellow': selectedPassword && selectedPassword.id === item.id}">
                  <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-key"><circle cx="7.5" cy="15.5" r="5.5"/><path d="m21 2-9.6 9.6"/><path d="m15.5 7.5 3 3L22 7l-3-3"/></svg>
                </div>
                <div class="min-w-0 flex-1">
                  <div class="font-black text-black dark:text-black uppercase tracking-tight text-base truncate" :title="item.name">{{ item.name }}</div>
                </div>
              </div>
            </div>

            <div v-if="filteredPasswords.length === 0" class="p-6 text-center rounded-none border-[3px] border-black bg-brut-white dark:bg-brut-dark-card dark:border-brut-white dark:text-white">
              <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-search-x mx-auto mb-3 text-black dark:text-brut-yellow"><path d="m13.5 8.5-5 5"/><path d="m8.5 8.5 5 5"/><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
              <p class="text-lg font-black uppercase">{{ t('emptyListTitle') }}</p>
              <p class="text-sm font-bold text-black/60 dark:text-white/60">{{ t('emptyListHint') }}</p>
            </div>
          </div>

        </div>
        <div class="shrink-0 px-6 py-4 flex space-x-3 border-t-[3px] border-black dark:border-brut-white">
          <!-- Settings button -->
          <button
              @click="showSettings = true"
              class="p-2 rounded-none border-[3px] border-black bg-brut-white dark:bg-brut-dark-card dark:border-brut-white text-black dark:text-white hover:bg-brut-magenta hover:text-black dark:hover:bg-brut-magenta dark:hover:text-black transition-colors shadow-[2px_2px_0_#000] dark:shadow-[2px_2px_0_#fff] active:shadow-none active:translate-x-[1px] active:translate-y-[1px]"
              :title="t('settingsBtnTitle')"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
          </button>
          <!-- Refresh button -->
          <button
              @click="refreshPasswords"
              class="p-2 rounded-none border-[3px] border-black bg-brut-white dark:bg-brut-dark-card dark:border-brut-white text-black dark:text-white hover:bg-brut-electric hover:text-black dark:hover:bg-brut-electric dark:hover:text-black transition-colors shadow-[2px_2px_0_#000] dark:shadow-[2px_2px_0_#fff] active:shadow-none active:translate-x-[1px] active:translate-y-[1px]"
              :title="t('refreshBtnTitle')"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-refresh-cw"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/></svg>
          </button>
        </div>
      </div>

      <!-- Resizable handle (pointer events + capture for smooth drag) -->
      <div
          class="w-2 shrink-0 cursor-col-resize touch-none select-none bg-black hover:bg-brut-magenta active:bg-brut-blue dark:bg-brut-white dark:hover:bg-brut-magenta dark:active:bg-brut-blue transition-colors"
          role="separator"
          aria-orientation="vertical"
          @pointerdown="startResize"
          @pointermove="onResizeMove"
          @pointerup="endResize"
          @pointercancel="endResize"
          @lostpointercapture="endResize"
      ></div>

      <!-- Right side - Password details -->
      <div class="flex-1 overflow-y-auto bg-brut-white dark:bg-brut-dark-bg">
        <div v-if="selectedPassword" class="password-details h-full">
          <div class="p-8 mx-auto">
            <div class="bg-brut-white dark:bg-brut-dark-card rounded-none p-6 border-[3px] border-black dark:border-brut-white shadow-[6px_6px_0_#000] dark:shadow-[6px_6px_0_#fff] space-y-6">
              <div class="bg-brut-blue text-white p-5 rounded-none border-[3px] border-black dark:border-brut-white mb-4"  @dblclick="copyPasswordToClipboard"
                   :title="t('copyHint')">
                <div class="text-xs font-black uppercase tracking-widest opacity-80 mb-1">{{ t('nameLabel') }}</div>
                <div class="text-3xl font-black uppercase tracking-tight break-words">{{ editedPassword?.name }}</div>
              </div>

              <div class="p-4 bg-brut-green rounded-none border-[3px] border-black dark:border-brut-white">
                <label class="block text-xs font-black uppercase tracking-widest text-black mb-2">{{ t('passwordLabel') }}</label>
                <div class="flex items-center justify-between gap-3">
                  <div class="text-lg text-black font-brut-mono font-bold break-all">
                    {{ showPassword ? (editedPassword?.password || t('noPassword')) : t('passwordHidden') }}
                  </div>
                  <div class="flex items-center shrink-0">
                    <button
                        @click="showPassword = !showPassword"
                        :title="showPassword ? t('hidePasswordHint') : t('showPasswordHint')"
                        :aria-label="showPassword ? t('hidePasswordHint') : t('showPasswordHint')"
                        :aria-pressed="showPassword"
                        class="bg-black hover:bg-brut-magenta text-white dark:bg-black dark:hover:bg-brut-magenta p-2 rounded-none border-2 border-black dark:border-brut-white transition-colors shadow-[2px_2px_0_#000] dark:shadow-[2px_2px_0_#fff] active:shadow-none active:translate-x-[1px] active:translate-y-[1px]"
                    >
                      <svg v-if="showPassword" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-eye-off"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>
                      <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-eye"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
                    </button>
                    <button
                        @click="copyPasswordToClipboard"
                        :title="t('copyWifiHint')"
                        :aria-label="t('copyWifiHint')"
                        class="ml-2 bg-brut-yellow hover:bg-brut-magenta text-black p-2 rounded-none border-2 border-black transition-colors shadow-[2px_2px_0_#000] active:shadow-none active:translate-x-[1px] active:translate-y-[1px]"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-copy"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>
                    </button>
                  </div>
                </div>
              </div>


              <!-- QR Code Section -->
              <div class="mt-8 p-6 bg-brut-gray dark:bg-brut-dark-bg rounded-none border-[3px] border-black dark:border-brut-white">
                <div class="grid grid-cols-1 md:grid-cols-1">
                  <!-- Username QR Code -->
                  <div class="flip-card h-[240px]">
                    <div class="flip-card-inner">
                      <!-- Front side -->
                      <div class="flip-card-front bg-brut-white dark:bg-brut-dark-card rounded-none border-[3px] border-black dark:border-brut-white">
                        <div class="h-full flex flex-col items-center justify-center">
                          <img
                              :src="generateQRCode"
                              alt="Username QR Code"
                              class="w-[200px] h-[200px] object-contain rounded-xl"
                              ref="usernameQRCode"
                          />
                        </div>
                      </div>

                      <!-- Back side -->
                      <div class="flip-card-back bg-brut-white dark:bg-brut-dark-card p-5 rounded-none border-[3px] border-black dark:border-brut-white">
                        <div class="h-full flex flex-col items-center justify-center space-y-6">
                          <button
                              @click.stop="copyQRCode"
                              class="bg-brut-yellow dark:bg-brut-yellow text-black border-[3px] border-black font-black py-3 px-6 rounded-none flex items-center uppercase tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-clipboard-copy mr-2"><rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"/><path d="M16 4h2a2 2 0 0 1 2 2v4"/><path d="M21 14H11"/><path d="m15 10-4 4 4 4"/></svg>                            {{ t('copyQR') }}
                          </button>
                          <button
                              @click.stop="downloadQRCode($refs.usernameQRCode as HTMLImageElement)"
                              class="bg-brut-magenta dark:bg-brut-magenta text-black border-[3px] border-black font-black py-3 px-6 rounded-none flex items-center uppercase tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-download mr-2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
                            {{ t('saveQR') }}
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="mt-4 text-center text-sm font-bold text-black dark:text-white">
                  <div class="flex items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-info mr-2 text-black dark:text-brut-yellow"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
                    {{ t('scanHint') }}
                  </div>
                </div>
              </div>

            </div>
          </div>
        </div>

        <div v-else class="flex flex-col items-center justify-center h-full">
          <div class="text-center p-8 max-w-md">
            <div class="bg-brut-magenta w-24 h-24 rounded-none border-[3px] border-black dark:border-brut-white flex items-center justify-center mx-auto mb-6 shadow-[6px_6px_0_#000] dark:shadow-[6px_6px_0_#fff]">
              <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-lock"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
            </div>
            <h3 class="text-3xl font-black uppercase tracking-tight text-black dark:text-white mb-2">{{ t('selectWifiTitle') }}</h3>
            <p class="font-bold text-black/70 dark:text-white/70">{{ t('selectWifiHint') }}</p>
          </div>
        </div>
      </div>
    </div>
    <!-- Refresh overlay/mask -->
    <div
        v-if="isRefreshing"
        class="fixed inset-0 bg-black/80 z-50 flex items-center justify-center"
    >
      <div class="bg-brut-white dark:bg-brut-dark-card rounded-none p-8 border-[4px] border-black dark:border-brut-white shadow-[8px_8px_0_#fff] dark:shadow-[8px_8px_0_#fff] flex flex-col items-center">
        <div class="w-16 h-16 mb-4 relative">
          <svg class="animate-spin text-brut-magenta" xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
          </svg>
        </div>
        <h3 class="text-2xl font-black uppercase text-black dark:text-white mb-2">{{ t('refreshingTitle') }}</h3>
        <p class="font-bold text-black/70 dark:text-white/70">{{ t('refreshingHint') }}</p>
      </div>
    </div>
    <!-- Toast notification -->
    <div
        v-if="showToast"
        class="fixed bottom-4 right-4 z-[100] bg-black text-white px-5 py-3 rounded-none border-[3px] border-white shadow-[4px_4px_0_#fff] flex items-center space-x-2 font-black uppercase tracking-tight"
        :class="{'opacity-100': showToast, 'opacity-0': !showToast}"
    >
      <svg v-if="toastType === 'success'" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check-circle text-brut-green"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
      <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-alert-circle text-brut-magenta"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>
      <span>{{ toastMessage }}</span>
      <button
          v-if="toastAction"
          @click="runToastAction"
          :aria-label="toastAction?.label"
          class="ml-2 bg-brut-yellow text-black border-[3px] border-black px-2 py-0.5 rounded-none font-black uppercase tracking-tight shadow-[2px_2px_0_#fff] hover:bg-brut-magenta transition-colors active:shadow-none active:translate-x-[1px] active:translate-y-[1px] shrink-0"
      >
        {{ toastAction?.label }}
      </button>
    </div>


    <!-- Settings Modal -->
    <div v-if="showSettings" class="fixed inset-0 bg-black/80 flex items-center justify-center z-50">
      <div class="bg-brut-white dark:bg-brut-dark-card rounded-none p-6 max-w-xl w-full mx-4 border-[4px] border-black dark:border-brut-white shadow-[8px_8px_0_#000] dark:shadow-[8px_8px_0_#fff]">
        <div class="flex justify-between items-center mb-6">
          <h3 class="text-2xl font-black uppercase tracking-tight text-black dark:text-white">{{ t('settings') }}</h3>
          <button
              @click="showSettings = false"
              class="text-black font-black hover:text-brut-magenta dark:text-white dark:hover:text-brut-magenta border-[3px] border-black dark:border-brut-white p-1 shadow-[2px_2px_0_#000] dark:shadow-[2px_2px_0_#fff] active:shadow-none active:translate-x-[1px] active:translate-y-[1px]"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
          </button>
        </div>

        <!-- Tabs -->
        <div class="mb-6 border-b-[3px] border-black dark:border-brut-white">
          <div class="flex flex-wrap gap-y-1">
            <button
                @click="activeSettingsTab = 'theme'"
                class="px-3 py-2 text-sm font-black uppercase tracking-tight transition-colors"
                :class="activeSettingsTab === 'theme' ? 'bg-brut-yellow text-black border-[3px] border-black -mb-[3px]' : 'text-black/60 hover:text-black dark:text-white/60 dark:hover:text-white'"
            >
              {{ t('tabTheme') }}
            </button>
            <button
                @click="activeSettingsTab = 'language'"
                class="px-3 py-2 text-sm font-black uppercase tracking-tight transition-colors"
                :class="activeSettingsTab === 'language' ? 'bg-brut-yellow text-black border-[3px] border-black -mb-[3px]' : 'text-black/60 hover:text-black dark:text-white/60 dark:hover:text-white'"
            >
              {{ t('tabLanguage') }}
            </button>
            <button
                @click="activeSettingsTab = 'backup'"
                class="px-3 py-2 text-sm font-black uppercase tracking-tight transition-colors"
                :class="activeSettingsTab === 'backup' ? 'bg-brut-yellow text-black border-[3px] border-black -mb-[3px]' : 'text-black/60 hover:text-black dark:text-white/60 dark:hover:text-white'"
            >
              {{ t('tabBackup') }}
            </button>
            <button
                @click="activeSettingsTab = 'donate'"
                class="px-3 py-2 text-sm font-black uppercase tracking-tight transition-colors"
                :class="activeSettingsTab === 'donate' ? 'bg-brut-yellow text-black border-[3px] border-black -mb-[3px]' : 'text-black/60 hover:text-black dark:text-white/60 dark:hover:text-white'"
            >
              {{ t('tabDonate') }}
            </button>
            <button
                @click="activeSettingsTab = 'about'"
                class="px-3 py-2 text-sm font-black uppercase tracking-tight transition-colors"
                :class="activeSettingsTab === 'about' ? 'bg-brut-yellow text-black border-[3px] border-black -mb-[3px]' : 'text-black/60 hover:text-black dark:text-white/60 dark:hover:text-white'"
            >
              {{ t('tabAbout') }}
            </button>
            <button
                @click="activeSettingsTab = 'update'"
                class="px-3 py-2 text-sm font-black uppercase tracking-tight transition-colors"
                :class="activeSettingsTab === 'update' ? 'bg-brut-yellow text-black border-[3px] border-black -mb-[3px]' : 'text-black/60 hover:text-black dark:text-white/60 dark:hover:text-white'"
            >
              {{ t('tabUpdate') }}
            </button>
          </div>
        </div>

        <!-- Theme Settings Tab -->
        <div v-if="activeSettingsTab === 'theme'" class="space-y-6">
          <div>
            <h4 class="font-black uppercase text-black dark:text-white mb-1">{{ t('themeTitle') }}</h4>
            <p class="text-sm font-bold text-black/60 dark:text-white/60 mb-4">{{ t('themeSubtitle') }}</p>
            <div class="grid grid-cols-3 gap-3">
              <!-- Light theme -->
              <button
                  @click="themeMode = 'light'"
                  class="flex flex-col items-center gap-3 p-3 rounded-none border-[3px] transition-all"
                  :class="themeMode === 'light'
                    ? 'border-black bg-brut-yellow shadow-[4px_4px_0_#000] text-black'
                    : 'border-black/40 bg-brut-white hover:border-black text-black/70 dark:bg-brut-dark-card dark:text-white/70 dark:border-brut-white/40 dark:hover:border-brut-white'"
              >
                <span class="w-12 h-12 rounded-none border-[3px] border-black bg-white flex items-center justify-center">
                  <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-sun"><circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/></svg>
                </span>
                <span class="font-black uppercase tracking-tight flex items-center gap-1">
                  {{ t('themeLight') }}
                  <svg v-if="themeMode === 'light'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><polyline points="20 6 9 17 4 12"/></svg>
                </span>
              </button>
              <!-- Dark theme -->
              <button
                  @click="themeMode = 'dark'"
                  class="flex flex-col items-center gap-3 p-3 rounded-none border-[3px] transition-all"
                  :class="themeMode === 'dark'
                    ? 'border-black bg-brut-magenta shadow-[4px_4px_0_#000] text-black'
                    : 'border-black/40 bg-brut-white hover:border-black text-black/70 dark:bg-brut-dark-card dark:text-white/70 dark:border-brut-white/40 dark:hover:border-brut-white'"
              >
                <span class="w-12 h-12 rounded-none border-[3px] border-black bg-gray-900 flex items-center justify-center text-white">
                  <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-moon"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/></svg>
                </span>
                <span class="font-black uppercase tracking-tight flex items-center gap-1">
                  {{ t('themeDark') }}
                  <svg v-if="themeMode === 'dark'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><polyline points="20 6 9 17 4 12"/></svg>
                </span>
              </button>
              <!-- Follow system -->
              <button
                  @click="themeMode = 'system'"
                  class="flex flex-col items-center gap-3 p-3 rounded-none border-[3px] transition-all"
                  :class="themeMode === 'system'
                    ? 'border-black bg-brut-blue shadow-[4px_4px_0_#000] text-white'
                    : 'border-black/40 bg-brut-white hover:border-black text-black/70 dark:bg-brut-dark-card dark:text-white/70 dark:border-brut-white/40 dark:hover:border-brut-white'"
              >
                <span class="w-12 h-12 rounded-none border-[3px] border-black bg-brut-electric flex items-center justify-center text-black">
                  <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-monitor"><rect width="20" height="14" x="2" y="3" rx="2"/><line x1="8" x2="16" y1="21" y2="21"/><line x1="12" x2="12" y1="17" y2="21"/></svg>
                </span>
                <span class="font-black uppercase tracking-tight flex items-center gap-1">
                  {{ t('themeSystem') }}
                  <svg v-if="themeMode === 'system'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><polyline points="20 6 9 17 4 12"/></svg>
                </span>
              </button>
            </div>
          </div>
        </div>

        <!-- Language Settings Tab -->
        <div v-if="activeSettingsTab === 'language'" class="space-y-6">
          <div>
            <h4 class="font-black uppercase text-black dark:text-white mb-1">{{ t('languageTitle') }}</h4>
            <p class="text-sm font-bold text-black/60 dark:text-white/60 mb-4">{{ t('languageSubtitle') }}</p>
            <div class="grid grid-cols-2 gap-4">
              <!-- Chinese -->
              <button
                  @click="locale = 'zh'"
                  class="flex flex-col items-center gap-3 p-4 rounded-none border-[3px] transition-all"
                  :class="locale === 'zh'
                    ? 'border-black bg-brut-yellow shadow-[4px_4px_0_#000] text-black'
                    : 'border-black/40 bg-brut-white hover:border-black text-black/70 dark:bg-brut-dark-card dark:text-white/70 dark:border-brut-white/40 dark:hover:border-brut-white'"
              >
                <span class="w-12 h-12 rounded-none border-[3px] border-black bg-brut-electric flex items-center justify-center font-black text-lg text-black">中</span>
                <span class="font-black uppercase tracking-tight flex items-center gap-1">
                  {{ t('langZh') }}
                  <svg v-if="locale === 'zh'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><polyline points="20 6 9 17 4 12"/></svg>
                </span>
              </button>
              <!-- English -->
              <button
                  @click="locale = 'en'"
                  class="flex flex-col items-center gap-3 p-4 rounded-none border-[3px] transition-all"
                  :class="locale === 'en'
                    ? 'border-black bg-brut-magenta shadow-[4px_4px_0_#000] text-black'
                    : 'border-black/40 bg-brut-white hover:border-black text-black/70 dark:bg-brut-dark-card dark:text-white/70 dark:border-brut-white/40 dark:hover:border-brut-white'"
              >
                <span class="w-12 h-12 rounded-none border-[3px] border-black bg-brut-green flex items-center justify-center font-black text-lg text-black">A</span>
                <span class="font-black uppercase tracking-tight flex items-center gap-1">
                  {{ t('langEn') }}
                  <svg v-if="locale === 'en'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><polyline points="20 6 9 17 4 12"/></svg>
                </span>
              </button>
            </div>
          </div>
        </div>

        <!-- Backup Settings Tab -->
        <div v-if="activeSettingsTab === 'backup'" class="space-y-6">
          <div class="border-[3px] border-black dark:border-brut-white p-4 bg-brut-white dark:bg-brut-dark-card shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff]">
            <h4 class="font-black uppercase text-black dark:text-white mb-1">{{ t('backupTitle') }}</h4>
            <p class="text-sm font-bold text-black/60 dark:text-white/60 mb-4">{{ t('backupSubtitle') }}</p>
            <div class="mb-4 inline-block border-[3px] border-black dark:border-brut-white bg-brut-yellow text-black px-3 py-1 font-black uppercase text-sm">
              {{ t('backupCount') }}: {{ wifiList.length }}
            </div>
            <div>
              <button
                  @click="exportWifiCsv"
                  :disabled="wifiList.length === 0"
                  class="bg-brut-green text-black border-[3px] border-black font-black uppercase py-2 px-6 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:shadow-none disabled:hover:translate-x-0 disabled:hover:translate-y-0 flex items-center"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-down mr-2"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M12 18v-6"/><path d="m9 15 3 3 3-3"/></svg>
                {{ t('backupExportBtn') }}
              </button>
            </div>
            <p class="mt-4 flex items-start text-xs font-bold text-black/70 dark:text-white/70">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-triangle-alert mr-2 shrink-0 text-brut-magenta"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="m10 15 2 2 2-2"/></svg>
              {{ t('backupWarning') }}
            </p>
          </div>
        </div>

        <!-- Update Settings Tab -->
        <div v-if="activeSettingsTab === 'update'" class="space-y-6">
          <div class="border-[3px] border-black dark:border-brut-white p-4 bg-brut-white dark:bg-brut-dark-card shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff]">
            <div class="flex items-center justify-between mb-3">
              <span class="text-xs font-black uppercase tracking-widest text-black/70 dark:text-white/70">{{ t('updateCurrent') }}</span>
              <span class="font-brut-mono font-black text-black dark:text-white">v{{ CURRENT_VERSION }}</span>
            </div>
            <div class="flex items-center justify-between mb-4">
              <span class="text-xs font-black uppercase tracking-widest text-black/70 dark:text-white/70">{{ t('updateLatest') }}</span>
              <span class="font-brut-mono font-black text-black dark:text-white">{{ newVersion ? 'v' + newVersion : '-' }}</span>
            </div>
            <div v-if="skippedVersion" class="mb-4 p-3 bg-brut-gray dark:bg-brut-dark-bg border-[3px] border-black dark:border-brut-white text-black dark:text-white text-sm font-bold flex items-center justify-between gap-3">
              <span class="uppercase">{{ t('updateSkippedHint') }} v{{ skippedVersion }}</span>
              <button
                  @click="clearSkippedVersion"
                  class="bg-brut-yellow text-black border-[3px] border-black font-black uppercase px-3 py-1 rounded-none text-xs shadow-[2px_2px_0_#000] active:shadow-none active:translate-x-[1px] active:translate-y-[1px] shrink-0"
              >
                {{ t('updateClearSkipBtn') }}
              </button>
            </div>
            <div class="flex flex-wrap gap-3">
              <button
                  @click="runUpdateCheck(false)"
                  :disabled="updateState === 'downloading'"
                  class="bg-brut-blue text-white border-[3px] border-black font-black uppercase py-2 px-4 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all disabled:opacity-60 disabled:cursor-not-allowed"
              >
                {{ t('updateCheck') }}
              </button>
              <button
                  v-if="updateState === 'ready'"
                  @click="restartApp"
                  class="bg-brut-green text-black border-[3px] border-black font-black uppercase py-2 px-4 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
              >
                {{ t('updateRestartBtn') }}
              </button>
              <button
                  @click="openGiteeReleases"
                  class="bg-brut-white dark:bg-brut-dark-card text-black dark:text-white border-[3px] border-black dark:border-brut-white font-black uppercase py-2 px-4 rounded-none tracking-tight shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] dark:hover:shadow-[5px_5px_0_#fff] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
              >
                {{ t('updateDownloadGitee') }}
              </button>
              <button
                  @click="openGithubReleases"
                  class="bg-brut-white dark:bg-brut-dark-card text-black dark:text-white border-[3px] border-black dark:border-brut-white font-black uppercase py-2 px-4 rounded-none tracking-tight shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] dark:hover:shadow-[5px_5px_0_#fff] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
              >
                {{ t('updateDownloadGithub') }}
              </button>
            </div>
            <p class="mt-3 text-xs font-bold text-black/60 dark:text-white/60">{{ t('updateFooter') }}</p>
          </div>
        </div>

        <!-- Donate Tab -->
        <div v-if="activeSettingsTab === 'donate'" class="space-y-6">
          <div>
            <h4 class="font-black uppercase text-black dark:text-white mb-2">{{ t('donateTitle') }}</h4>
            <p class="text-sm font-bold text-black/60 dark:text-white/60 mb-6">{{ t('donateSubtitle') }}</p>

            <div class="grid grid-cols-2 gap-4">

              <!-- Alipay QR Code -->
              <div class="bg-brut-blue rounded-none p-4 border-[3px] border-black dark:border-brut-white">
                <div class="flex flex-col items-center space-y-4">
                  <div class="flex space-x-2 justify-center">
                    <div class="w-6 h-6 bg-black border-2 border-black flex items-center justify-center">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.18 8H12l-1.8 5.37.01.01C13.5 13.95 15 15 17.5 15a8.7 8.7 0 0 0 3.5-.7"/><path d="M9 11h6"/><path d="M12 21a9 9 0 1 1 0-18 9 9 0 0 1 0 18Z"/></svg>
                    </div >
                    <div class="flex justify-center items-center">
                      <h5 class="font-black uppercase text-white">{{ t('donateAlipay') }}</h5>
                    </div>
                  </div>
                  <div class="bg-white border-[3px] border-black p-2 mb-3">
                    <img
                        src="./assets/zhifubao.jpg"
                        :alt="t('donateAlipay')"
                        class="w-38 h-38 object-cover rounded-none"
                    />
                  </div>
                </div>
              </div>

              <!-- WeChat QR Code -->
              <div class="bg-brut-green rounded-none p-4 border-[3px] border-black dark:border-brut-white">
                <div class="flex flex-col items-center space-y-4">
                  <div class="flex space-x-2 justify-center">
                    <div class="w-6 h-6 bg-black border-2 border-black flex items-center justify-center">
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"/></svg>
                    </div>
                    <div class="flex justify-center items-center">
                      <h5 class="font-black uppercase text-black">{{ t('donateWechat') }}</h5>
                    </div>
                  </div>
                  <div class="bg-white border-[3px] border-black p-2 mb-3">
                    <img
                        src="./assets/weixin.png"
                        :alt="t('donateWechat')"
                        class="w-38 h-38 object-cover rounded-none"
                    />
                  </div>
                </div>
              </div>
            </div>

            <div class="mt-6 text-center text-sm font-bold text-black dark:text-white flex items-center justify-center">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 text-brut-magenta"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
              {{ t('donateFoot') }}
            </div>
          </div>
        </div>

        <!-- About Tab -->
        <div v-if="activeSettingsTab === 'about'" class="space-y-5">
          <!-- 身份贴纸：图标 + 品牌 + 口号 + 版本徽章 -->
          <div class="border-[3px] border-black dark:border-brut-white bg-brut-yellow text-black p-4 shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] flex items-center gap-4">
            <img src="../assets/app-icon.png" alt="" class="w-14 h-14 shrink-0 border-[3px] border-black bg-brut-white" />
            <div class="flex-1 min-w-0">
              <div class="text-xl font-black uppercase tracking-tight truncate">{{ t('aboutBrand') }}</div>
              <div class="text-xs font-black uppercase tracking-widest opacity-70 truncate">{{ t('appTitle') }}</div>
            </div>
            <div class="shrink-0 text-center">
              <div class="text-[9px] font-black tracking-widest mb-0.5">VERSION</div>
              <div class="border-[3px] border-black bg-brut-white px-2 py-0.5 font-brut-mono font-black text-sm shadow-[3px_3px_0_#000]">v{{ CURRENT_VERSION }}</div>
            </div>
          </div>

          <!-- 仓库卡片：GitHub / Gitee 整卡可点 -->
          <div class="grid grid-cols-2 gap-4">
            <button
                @click="openGithubRepo"
                class="group flex items-center gap-3 border-[3px] border-black dark:border-brut-white bg-brut-white dark:bg-brut-dark-card p-3 text-left shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] dark:hover:shadow-[5px_5px_0_#fff] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              <span class="w-11 h-11 shrink-0 border-[3px] border-black dark:border-brut-white bg-black text-white flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-github"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/><path d="M9 18c-4.51 2-5-2-7-2"/></svg>
              </span>
              <span class="flex-1 min-w-0">
                <span class="block font-black uppercase tracking-tight text-black dark:text-white">GitHub</span>
                <span class="block text-[10px] font-brut-mono text-black/60 dark:text-white/60 truncate">ShiXiongZhiDao/PlainWiFi</span>
              </span>
              <span class="font-black text-lg text-black/60 dark:text-white/60 group-hover:text-brut-magenta shrink-0">↗</span>
            </button>
            <button
                @click="openGiteeRepo"
                class="group flex items-center gap-3 border-[3px] border-black dark:border-brut-white bg-brut-white dark:bg-brut-dark-card p-3 text-left shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] dark:hover:shadow-[5px_5px_0_#fff] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              <span class="w-11 h-11 shrink-0 border-[3px] border-black dark:border-brut-white bg-brut-magenta text-black flex items-center justify-center font-black text-xl">码</span>
              <span class="flex-1 min-w-0">
                <span class="block font-black uppercase tracking-tight text-black dark:text-white">Gitee</span>
                <span class="block text-[10px] font-brut-mono text-black/60 dark:text-white/60 truncate">ShiXiongZhiDao/PlainWiFi</span>
              </span>
              <span class="font-black text-lg text-black/60 dark:text-white/60 group-hover:text-brut-magenta shrink-0">↗</span>
            </button>
          </div>

          <!-- 作者信息条：硬边框分栏 -->
          <div class="border-[3px] border-black dark:border-brut-white bg-brut-white dark:bg-brut-dark-card shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] divide-y-[3px] divide-black dark:divide-brut-white">
            <div class="flex items-center justify-between px-4 py-2.5">
              <span class="text-xs font-black uppercase tracking-widest text-black/60 dark:text-white/60">{{ t('aboutAuthor') }}</span>
              <span class="font-black text-black dark:text-white">{{ locale === 'zh' ? '师兄知道' : 'ShiXiongZhiDao' }}</span>
            </div>
            <div class="flex items-center justify-between px-4 py-2.5">
              <span class="text-xs font-black uppercase tracking-widest text-black/60 dark:text-white/60">{{ t('aboutWechatMP') }}</span>
              <span class="font-black text-black dark:text-white">{{ locale === 'zh' ? '师兄知道' : 'ShiXiongZhiDao' }}</span>
            </div>
          </div>

          <!-- 技术栈脚注 -->
          <div class="text-center text-[10px] font-brut-mono text-black/50 dark:text-white/50 tracking-wider">
            TAURI 2 · VUE 3 · TAILWIND 4 · MIT © 2026 师兄知道
          </div>
        </div>

        <div class="mt-8 flex justify-end">
          <button
              @click="showSettings = false"
              class="bg-brut-yellow hover:bg-brut-magenta text-black font-black uppercase py-2 px-6 rounded-none border-[3px] border-black tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
          >
            {{ t('btnOk') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Update Confirm Dialog（z-[90]：盖过设置弹窗 z-50，低于 Toast z-[100]） -->
    <div v-if="updateDialogOpen" class="fixed inset-0 bg-black/80 flex items-center justify-center z-[90]">
      <div class="bg-brut-white dark:bg-brut-dark-card rounded-none p-6 max-w-md w-full mx-4 border-[4px] border-black dark:border-brut-white shadow-[8px_8px_0_#000] dark:shadow-[8px_8px_0_#fff]">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-xl font-black uppercase tracking-tight text-black dark:text-white">
            {{ t('updateAvailable') }} <span class="text-brut-magenta">v{{ newVersion }}</span>
          </h3>
          <button
              @click="closeUpdateDialog"
              class="text-black font-black hover:text-brut-magenta dark:text-white dark:hover:text-brut-magenta border-[3px] border-black dark:border-brut-white p-1 shadow-[2px_2px_0_#000] dark:shadow-[2px_2px_0_#fff] active:shadow-none active:translate-x-[1px] active:translate-y-[1px]"
              :aria-label="t('updateBtnLater')"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-x"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
          </button>
        </div>

        <!-- 待确认：展示更新内容 + 更新/跳过/取消 -->
        <template v-if="updateState === 'available'">
          <div class="text-xs font-black uppercase tracking-widest text-black/60 dark:text-white/60 mb-1">{{ t('updateNotesTitle') }}</div>
          <div class="max-h-48 overflow-y-auto border-[3px] border-black dark:border-brut-white p-3 mb-5 bg-brut-gray dark:bg-brut-dark-bg text-sm font-bold text-black dark:text-white whitespace-pre-wrap break-words">{{ updateNotes || t('updateNotesEmpty') }}</div>
          <div class="flex flex-wrap gap-3">
            <button
                @click="acceptUpdate"
                class="bg-brut-green text-black border-[3px] border-black font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateBtnNow') }}
            </button>
            <button
                @click="skipUpdate"
                class="bg-brut-white dark:bg-brut-dark-card text-black dark:text-white border-[3px] border-black dark:border-brut-white font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] dark:hover:shadow-[5px_5px_0_#fff] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateBtnSkip') }}
            </button>
            <button
                @click="closeUpdateDialog"
                class="bg-brut-yellow text-black border-[3px] border-black font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateBtnLater') }}
            </button>
          </div>
        </template>

        <!-- 下载中：进度 -->
        <template v-else-if="updateState === 'downloading'">
          <div class="h-6 border-[3px] border-black dark:border-brut-white bg-brut-white dark:bg-brut-dark-bg mb-2">
            <div class="h-full bg-brut-electric transition-all" :style="{ width: `${updateProgress}%` }"></div>
          </div>
          <div class="text-sm font-black text-black dark:text-white">{{ t('updateDownloading') }} {{ updateProgress }}%</div>
        </template>

        <!-- 就绪：重启 -->
        <template v-else-if="updateState === 'ready'">
          <div class="mb-4 p-3 bg-brut-green border-[3px] border-black text-black font-black uppercase text-sm shadow-[3px_3px_0_#000]">{{ t('updateInstalled') }}</div>
          <div class="flex gap-3">
            <button
                @click="restartApp"
                class="bg-brut-magenta text-black border-[3px] border-black font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateRestartBtn') }}
            </button>
            <button
                @click="closeUpdateDialog"
                class="bg-brut-white dark:bg-brut-dark-card text-black dark:text-white border-[3px] border-black dark:border-brut-white font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] dark:shadow-[4px_4px_0_#fff] active:shadow-none active:translate-x-[2px] active:translate-y-[2px] transition-all"
            >
              {{ t('btnOk') }}
            </button>
          </div>
        </template>

        <!-- 失败：重试 / 下载页 -->
        <template v-else>
          <div class="mb-4 p-3 bg-brut-magenta border-[3px] border-black text-black font-black uppercase text-sm shadow-[3px_3px_0_#000]">{{ t('updateError') }}</div>
          <div class="flex flex-wrap gap-3">
            <button
                v-if="pendingUpdate"
                @click="acceptUpdate"
                class="bg-brut-blue text-white border-[3px] border-black font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateRetryBtn') }}
            </button>
            <button
                @click="openGiteeReleases"
                class="bg-brut-yellow text-black border-[3px] border-black font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateDownloadGitee') }}
            </button>
            <button
                @click="openGithubReleases"
                class="bg-brut-blue text-white border-[3px] border-black font-black uppercase py-2 px-5 rounded-none tracking-tight shadow-[4px_4px_0_#000] hover:translate-x-[-1px] hover:translate-y-[-1px] hover:shadow-[5px_5px_0_#000] active:translate-x-[3px] active:translate-y-[3px] active:shadow-none transition-all"
            >
              {{ t('updateDownloadGithub') }}
            </button>
          </div>
        </template>
      </div>
    </div>

  </div>
</template>


<script setup lang="ts">
import {computed, onMounted, ref, watch} from 'vue'
import { emit as emitTauriEvent } from '@tauri-apps/api/event'
import QRCode from 'qrcode';
import { Command } from '@tauri-apps/plugin-shell'
import { openUrl, revealItemInDir } from '@tauri-apps/plugin-opener'
import { downloadDir, join } from '@tauri-apps/api/path'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

// ============ i18n ============
type Locale = 'zh' | 'en'
const translations: Record<Locale, Record<string, string>> = {
  zh: {
    appTitle: 'WiFi密码不打码',
    searchPlaceholder: '搜索WiFi...',
    clearSearchAria: '清除搜索',
    emptyListTitle: '没有找到WiFi',
    emptyListHint: '尝试其他搜索词',
    settingsBtnTitle: '设置',
    refreshBtnTitle: '刷新WiFi列表',
    passwordHidden: '••••••••••••',
    nameLabel: '名称',
    passwordLabel: '密码',
    copyHint: '双击复制密码',
    showPasswordHint: '显示密码',
    hidePasswordHint: '隐藏密码',
    copyWifiHint: '一键复制 WiFi 名称与密码',
    copyQR: '复制二维码',
    saveQR: '保存二维码',
    openFolder: '打开文件夹',
    scanHint: '使用手机扫描二维码，快速连接WiFi',
    selectWifiTitle: '选择一个WiFi',
    selectWifiHint: '从左侧列表中选择一个WiFi查看详情',
    refreshingTitle: '正在刷新WiFi列表',
    refreshingHint: '请稍候...',
    settings: '设置',
    tabTheme: '主题',
    tabLanguage: '语言',
    tabDonate: '打赏',
    tabAbout: '关于',
    tabUpdate: '升级',
    themeTitle: '界面主题',
    themeSubtitle: '选择浅色、深色或跟随系统',
    themeLight: '浅色',
    themeDark: '深色',
    themeSystem: '跟随系统',
    languageTitle: '界面语言',
    languageSubtitle: '选择显示语言 / Choose display language',
    langZh: '中文',
    langEn: 'English',
    tabBackup: '备份',
    backupTitle: '备份 WiFi 配置',
    backupSubtitle: '将本机已保存的 WiFi 名称与密码导出为 CSV 文件，便于备份与迁移',
    backupCount: '可导出网络数',
    backupExportBtn: '导出 CSV',
    backupWarning: '导出文件包含明文密码，请妥善保管；将文件分享给他人即等于交出对应网络的访问权限',
    backupColAuth: '身份验证',
    backupColHidden: '隐藏网络',
    yes: '是',
    no: '否',
    notifyExported: 'WiFi 配置已导出',
    notifyExportEmpty: '暂无可导出的 WiFi，请先刷新列表',
    notifyExportFailed: '导出失败，请重试',
    donateTitle: '打赏',
    donateSubtitle: '如果您觉得这个应用有用，可以考虑支持开发者',
    donateAlipay: '支付宝',
    donateWechat: '微信支付',
    donateFoot: '一分也是爱！您的支持是我们持续改进的动力',
    aboutAuthor: '作者',
    aboutWechatMP: '微信公众号',
    aboutBrand: '师兄明码WiFi',
    btnOk: '确定',
    updateCurrent: '当前版本',
    updateLatest: '最新版本',
    updateCheck: '检查更新',
    updateDownloadGitee: '前往下载页 (Gitee)',
    updateDownloadGithub: '前往下载页 (GitHub)',
    updateUpToDate: '已是最新版本',
    updateAvailable: '发现新版本',
    updateError: '升级失败，请检查网络或重试',
    updateFooter: '启动时后台检查；发现新版弹窗展示更新内容，由你选择立即更新、跳过此版本或稍后再说',
    updateInstalled: '新版本已安装，重启生效',
    updateRestartBtn: '立即重启',
    updateDownloading: '正在下载安装',
    updateRelaunchFailed: '重启失败，请手动重启应用',
    updateNotesTitle: '更新内容',
    updateBtnNow: '立即更新',
    updateBtnSkip: '跳过此版本',
    updateBtnLater: '稍后再说',
    updateRetryBtn: '重试',
    updateNotesEmpty: '该版本未提供更新说明',
    updateSkippedHint: '已跳过版本',
    updateClearSkipBtn: '恢复提醒',
    notifyGetWifiFailed: '获取Wi-Fi密码失败: ',
    notifyCopied: '密码已复制到剪贴板',
    notifyCopyFailedRetry: '复制密码失败，请重试',
    notifyPleaseSelectWifi: '请选择一个WiFi',
    notifyNoQr: '没有可用的二维码，请选择一个WiFi',
    notifyQrCopied: '二维码已复制到剪贴板',
    notifyQrCopyFailed: '复制二维码失败，请重试',
    notifyQrSaved: '二维码已保存到「下载」文件夹',
    notifyQrSaveFailed: '二维码保存失败，请重试',
    notifyRevealFailed: '无法打开所在文件夹',
    notifyRefreshFailed: '刷新WiFi列表失败，请重试',
    noPassword: '无密码',
    clipboardWifiLabel: 'WIFI',
    clipboardPwdLabel: '密码',
  },
  en: {
    appTitle: 'YOUR WIFI. NO SECRETS.',
    searchPlaceholder: 'Search WiFi...',
    clearSearchAria: 'Clear search',
    emptyListTitle: 'No WiFi Found',
    emptyListHint: 'Try other keywords',
    settingsBtnTitle: 'Settings',
    refreshBtnTitle: 'Refresh WiFi List',
    passwordHidden: '••••••••••••',
    nameLabel: 'Name',
    passwordLabel: 'Password',
    copyHint: 'Double-click to copy password',
    showPasswordHint: 'Show password',
    hidePasswordHint: 'Hide password',
    copyWifiHint: 'Copy WiFi name & password',
    copyQR: 'Copy QR Code',
    saveQR: 'Save QR Code',
    openFolder: 'Open Folder',
    scanHint: 'Scan the QR code with your phone to join WiFi',
    selectWifiTitle: 'Select a WiFi',
    selectWifiHint: 'Pick one from the left list to view details',
    refreshingTitle: 'Refreshing WiFi List',
    refreshingHint: 'Please wait...',
    settings: 'Settings',
    tabTheme: 'Theme',
    tabLanguage: 'Language',
    tabDonate: 'Tip',
    tabAbout: 'About',
    tabUpdate: 'Update',
    themeTitle: 'Interface Theme',
    themeSubtitle: 'Choose light, dark, or follow the system',
    themeLight: 'Light',
    themeDark: 'Dark',
    themeSystem: 'System',
    languageTitle: 'Interface Language',
    languageSubtitle: '选择显示语言 / Choose display language',
    langZh: '中文',
    langEn: 'English',
    tabBackup: 'Backup',
    backupTitle: 'Backup WiFi Profiles',
    backupSubtitle: 'Export saved WiFi names and passwords to a CSV file for backup or migration',
    backupCount: 'Exportable networks',
    backupExportBtn: 'Export CSV',
    backupWarning: 'The exported file contains plaintext passwords. Store it safely — sharing it grants others access to these networks',
    backupColAuth: 'Authentication',
    backupColHidden: 'Hidden',
    yes: 'Yes',
    no: 'No',
    notifyExported: 'WiFi profiles exported',
    notifyExportEmpty: 'No WiFi to export, refresh the list first',
    notifyExportFailed: 'Export failed, please retry',
    donateTitle: 'Tip',
    donateSubtitle: 'If you find this app useful, consider supporting the developer',
    donateAlipay: 'Alipay',
    donateWechat: 'WeChat Pay',
    donateFoot: 'Every bit counts! Your support keeps us improving',
    aboutAuthor: 'Author',
    aboutWechatMP: 'WeChat Official Account',
    aboutBrand: 'PlainWiFi',
    btnOk: 'OK',
    updateCurrent: 'Current Version',
    updateLatest: 'Latest Version',
    updateCheck: 'Check for Updates',
    updateDownloadGitee: 'Download (Gitee)',
    updateDownloadGithub: 'Download (GitHub)',
    updateUpToDate: 'Already up to date',
    updateAvailable: 'Update available',
    updateError: 'Update failed. Check your network and retry.',
    updateFooter: 'Checked silently at startup; when a new version is found a dialog shows the release notes and you choose Update now / Skip / Later',
    updateInstalled: 'Update installed — restart to apply',
    updateRestartBtn: 'Restart Now',
    updateDownloading: 'Downloading & installing',
    updateRelaunchFailed: 'Relaunch failed, please restart manually',
    updateNotesTitle: 'Release Notes',
    updateBtnNow: 'Update Now',
    updateBtnSkip: 'Skip This Version',
    updateBtnLater: 'Later',
    updateRetryBtn: 'Retry',
    updateNotesEmpty: 'No release notes provided for this version',
    updateSkippedHint: 'Skipped version',
    updateClearSkipBtn: 'Un-skip',
    notifyGetWifiFailed: 'Failed to get Wi-Fi passwords: ',
    notifyCopied: 'Password copied to clipboard',
    notifyCopyFailedRetry: 'Copy failed, please retry',
    notifyPleaseSelectWifi: 'Please select a WiFi',
    notifyNoQr: 'No QR code available, please select a WiFi',
    notifyQrCopied: 'QR code copied to clipboard',
    notifyQrCopyFailed: 'QR copy failed, please retry',
    notifyQrSaved: 'QR code saved to your Downloads folder',
    notifyQrSaveFailed: 'Failed to save QR code, please retry',
    notifyRevealFailed: 'Could not open the folder',
    notifyRefreshFailed: 'Refresh failed, please retry',
    noPassword: 'No password',
    clipboardWifiLabel: 'WIFI',
    clipboardPwdLabel: 'Password',
  },
}
const locale = ref<Locale>('zh')
const t = (key: string): string => translations[locale.value][key] ?? key

// ============ In-app update with user confirmation (tauri-plugin-updater) ============
const CURRENT_VERSION = '1.0.2'
// 升级清单地址由 tauri.conf.json → plugins.updater.endpoints 配置（check() 在 Rust 侧读取，Gitee 优先、GitHub 兜底）
const GITEE_REPO_URL = 'https://gitee.com/ShiXiongZhiDao/PlainWiFi'
const GITHUB_REPO_URL = 'https://github.com/ShiXiongZhiDao/PlainWiFi'
const UPDATE_SKIPPED_KEY = 'updateSkippedVersion'

type UpdateState = 'idle' | 'available' | 'downloading' | 'ready' | 'error'
const updateState = ref<UpdateState>('idle')
const updateProgress = ref(0)
const newVersion = ref('')
const updateNotes = ref('')
const updateDialogOpen = ref(false)
const skippedVersion = ref(localStorage.getItem(UPDATE_SKIPPED_KEY) ?? '')
let pendingUpdate: Update | null = null

// 从 updater 协议 body 中提取更新说明（当前版本为 string；兼容旧版按平台对象形式）
function extractUpdateNotes(u: Update): string {
  const b = u.body as unknown
  if (typeof b === 'string') return b
  if (b && typeof b === 'object') {
    const first = Object.values(b as Record<string, { notes?: string }>)[0]
    return first?.notes ?? ''
  }
  return ''
}

// 检查更新：启动时 silent=true（失败不打扰、被跳过的版本不再弹窗）；手动检查永远弹窗/提示
async function runUpdateCheck(silent: boolean) {
  if (updateState.value === 'downloading') return
  try {
    const update = await check()
    if (!update || !update.available) {
      if (!silent) showNotification(t('updateUpToDate'), 'success')
      return
    }
    if (silent && skippedVersion.value === update.version) return
    pendingUpdate = update
    newVersion.value = update.version
    updateNotes.value = extractUpdateNotes(update)
    updateState.value = 'available'
    updateDialogOpen.value = true
  } catch (err) {
    console.error('updater check failed:', err)
    updateState.value = 'error'
    updateDialogOpen.value = true
    if (!silent) showNotification(t('updateError'), 'error')
  }
}

// 用户确认「立即更新」后才下载安装
async function acceptUpdate() {
  if (!pendingUpdate) return
  updateState.value = 'downloading'
  updateProgress.value = 0
  let downloaded = 0
  let total = 0
  try {
    await pendingUpdate.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          total = event.data.contentLength ?? 0
          break
        case 'Progress':
          downloaded += event.data.chunkLength
          if (total > 0) updateProgress.value = Math.min(99, Math.floor((downloaded / total) * 100))
          break
        case 'Finished':
          updateProgress.value = 100
          break
      }
    })
    updateState.value = 'ready'
    showNotification(t('updateInstalled'), 'success')
  } catch (err) {
    console.error('install failed:', err)
    updateState.value = 'error'
    showNotification(t('updateError'), 'error')
  }
}

// 跳过此版本：持久化版本号，后续启动自动检查不再打扰；手动检查仍可命中
function skipUpdate() {
  if (newVersion.value) {
    localStorage.setItem(UPDATE_SKIPPED_KEY, newVersion.value)
    skippedVersion.value = newVersion.value
  }
  closeUpdateDialog()
}

function clearSkippedVersion() {
  localStorage.removeItem(UPDATE_SKIPPED_KEY)
  skippedVersion.value = ''
}

function closeUpdateDialog() {
  updateDialogOpen.value = false
  if (updateState.value === 'available' || updateState.value === 'error') updateState.value = 'idle'
}

async function restartApp() {
  try {
    await relaunch()
  } catch (err) {
    console.error('relaunch failed:', err)
    showNotification(t('updateRelaunchFailed'), 'error')
  }
}

// 统一外链打开：走系统浏览器，避免 webview 内导航
async function openExternal(url: string) {
  try {
    await openUrl(url)
  } catch (err) {
    console.error('open url failed:', err)
  }
}

const openGiteeRepo = () => openExternal(GITEE_REPO_URL)
const openGithubRepo = () => openExternal(GITHUB_REPO_URL)
const openGiteeReleases = () => openExternal(`${GITEE_REPO_URL}/releases`)
const openGithubReleases = () => openExternal(`${GITHUB_REPO_URL}/releases`)

// UI state
const selectedPassword = ref<Wifi>()
const editedPassword = ref<Wifi>()
const showPassword = ref(false)
const searchKey = ref('')

// Toast notification state
const showToast = ref(false)
const toastMessage = ref('')
const toastType = ref('success')
// toast 可选操作按钮（如“保存成功后打开所在文件夹”）
interface ToastAction { label: string; run: () => void }
const toastAction = ref<ToastAction | null>(null)
let toastTimer: ReturnType<typeof setTimeout> | undefined
interface Wifi{
  id: number;
  name: string;
  password: string;
  src: string;
  auth: string;   // netsh 输出的身份验证类型（WPA2-PSK / WEP / 开放…）
  hidden: boolean; // 广播 SSID 设置是否为禁用（隐藏网络）
}
// 更改变量名
const wifiList = ref<Wifi[]>([]);
// Refresh state
const isRefreshing = ref(false)
// Settings state
const showSettings = ref(false)
// 主题三态模式：light / dark / system（跟随系统），isDarkMode 为派生只读值
type ThemeMode = 'light' | 'dark' | 'system'
const themeMode = ref<ThemeMode>('light')
const systemDark = ref(false)
const isDarkMode = computed(() =>
    themeMode.value === 'dark' || (themeMode.value === 'system' && systemDark.value)
)
const activeSettingsTab = ref('theme') // 设置页面的活动标签，默认展示主题选择

// Sidebar resize state
const sidebarWidth = ref(336) // 960 × 35%（ADR-0004 修订）
const isResizing = ref(false)
const SIDEBAR_MIN = 216
const SIDEBAR_MAX = 600
// 拖拽内部状态（非响应式，避免无谓重渲染）
let resizeStartX = 0
let resizeStartWidth = 0
let pendingWidth: number | null = null
let resizeRafId = 0
let activePointerId: number | null = null

// Computed property for filtered passwords
const filteredPasswords = computed(() => {
  if (!searchKey.value) return wifiList.value

  const term = searchKey.value.toLowerCase()
  return wifiList.value.filter(password =>
      password.name.toLowerCase().includes(term)
  )
})

// 归一化 SSID 作为去重键：剥离零宽/不可见字符（GBK 解码后常见），统一空白，忽略大小写
function normalizeSsidKey(ssid: string): string {
  return ssid
      .replace(/[\u200B-\u200F\u202A-\u202E\u2060\uFEFF\u00A0]/g, '')
      .trim()
      .toLowerCase();
}

async function getWifiPasswords() {
  wifiList.value = []
  try {
    const command = Command.create('exec-netsh', ['wlan', 'show', 'profiles'], {
      encoding: "gbk",
      // encoding: "utf-8",
    });
    const { code, stdout, stderr } = await command.execute();
    console.log("code", code);
    console.log("stdout", stdout);
    console.log("stderr", stderr);
    if (code != 0) {
      console.error("code", code);
      console.error("stderr", stderr);
      showNotification(t('notifyGetWifiFailed') + stderr, 'error');
      return;
    }
    try {
      // 双语兼容 + 用 [ \t] 收紧空白，避免 \s 跨行把下一行内容误捕获为 SSID
      const profiles = stdout.match(/(?:所有用户配置文件|All User Profile)[ \t]*:[ \t]*(.*)/gi);
      console.log("profiles", profiles);
      if (profiles) {
        // 同一配置文件会在多个 WLAN 接口（物理网卡/移动热点虚拟网卡）下各列一次，按 SSID 去重
        const seenSsids = new Set<string>();
        for (const profile of profiles) {
          const matchResult = profile.match(/(?:所有用户配置文件|All User Profile)[ \t]*:[ \t]*(.*)/i);
          if (matchResult === undefined || matchResult === null){
            continue
          }
          const ssid = matchResult[1].trim();
          const ssidKey = normalizeSsidKey(ssid);
          if (!ssidKey) {
            continue
          }
          if (seenSsids.has(ssidKey)) {
            // 打印原始值，便于排查不可见字符差异
            console.warn('跳过重复WiFi:', JSON.stringify(ssid));
            continue
          }
          seenSsids.add(ssidKey);

          // 不可以使用双引号，否则会获取不到密码
          // const ssidOption = `name="${ssid}"`;
          const ssidOption = `name=${ssid}`

          // 对 SSID 进行引号处理
          console.log("ssid", ssidOption);
          const passwordCommand = Command.create('exec-netsh', ['wlan', 'show', 'profile', ssidOption, 'key=clear'], {
            encoding: "gbk",
            // encoding: "utf-8",
          });

          const {code: pwdCode, stdout: passwordOutput, stderr: pwdErr} = await passwordCommand.execute();
          // 单条查询失败时跳过，避免把“查询失败”误判为“开放网络”生成 nopass 二维码
          if (pwdCode !== 0) {
            console.warn(`查询 "${ssid}" 失败(code=${pwdCode})，已跳过:`, pwdErr);
            continue;
          }
          // 密码：无 key=clear 输出（开放/企业网络）时存空字符串，展示层再映射文案
          const passwordMatch = passwordOutput.match(/(?:关键内容|Security key)\s*[:：]\s*(.*)/i);
          const password = passwordMatch ? passwordMatch[1].trim() : '';
          // 加密类型：兼容中英文系统 locale 的 netsh 输出
          const authMatch = passwordOutput.match(/(?:身份验证|Authentication)\s*[:：]\s*(.*)/i);
          const auth = authMatch ? authMatch[1].trim() : '';
          // 隐藏网络：广播 SSID 设置为“禁用/No”时 hidden=true
          const broadcastMatch = passwordOutput.match(/(?:广播\s*SSID\s*设置|Broadcast\s*SSID)\s*[:：]\s*(.*)/i);
          const broadcast = broadcastMatch ? broadcastMatch[1].trim() : '';
          const hidden = broadcast.includes('禁用') || /^no\b/i.test(broadcast);
          wifiList.value.push({
            id: wifiList.value.length + 1,
            name: ssid,
            password: password,
            src: '',
            auth: auth,
            hidden: hidden,
          });
        }
        // 刷新后按 SSID 重绑选中项，避免旧对象悬空、二维码 src 写到 id 相同的其它条目上
        if (selectedPassword.value) {
          const prevKey = normalizeSsidKey(selectedPassword.value.name);
          const rebound = wifiList.value.find(item => normalizeSsidKey(item.name) === prevKey);
          selectedPassword.value = rebound;
          editedPassword.value = rebound ? { ...rebound } : undefined;
          if (!rebound) showPassword.value = false;
        }
      }
    } catch (error){
      console.error('获取 WiFi 密码时发生错误:', error);
    }
  } catch (error) {
    console.error('获取 WiFi 密码时发生错误:', error);
  }
}

// Show toast notification（带 action 时停留更久，留出阅读与点击时间）
const showNotification = (message: string, type = 'success', action: ToastAction | null = null) => {
  toastMessage.value = message
  toastType.value = type
  toastAction.value = action
  showToast.value = true

  // 清除上一条残留计时，否则旧定时器会提前掉掉新 toast
  if (toastTimer) clearTimeout(toastTimer)
  toastTimer = setTimeout(() => {
    showToast.value = false
    toastAction.value = null
  }, action ? 6000 : 3000)
}

// 点击 toast 动作按钮：先收起再执行
const runToastAction = () => {
  const action = toastAction.value
  showToast.value = false
  toastAction.value = null
  if (toastTimer) clearTimeout(toastTimer)
  action?.run()
}

// Copy password to clipboard
const copyPasswordToClipboard = async () => {
  try {
    // If password is hidden, show it temporarily
    const wasHidden = !showPassword.value;
    if (wasHidden) {
      showPassword.value = true;
    }
    if (selectedPassword.value?.password === undefined) {
      showNotification(t('notifyPleaseSelectWifi'), 'error');
      return;
    }
    const template = `
    ${t('clipboardWifiLabel')}: ${selectedPassword.value?.name}
    ${t('clipboardPwdLabel')}: ${selectedPassword.value?.password || t('noPassword')}
    `
    // Copy to clipboard

    await navigator.clipboard.writeText(template);

    // Show success notification
    showNotification(t('notifyCopied'));

    // If password was hidden before, hide it again after a short delay
    if (wasHidden) {
      setTimeout(() => {
        showPassword.value = false;
      }, 1500);
    }
  } catch (err) {
    console.error('copy password failed:', err);
    showNotification(t('notifyCopyFailedRetry'), 'error');
  }
}

// data URL → 二进制字节（复制二维码 / 保存二维码共用）
function dataUrlToUint8Array(dataUrl: string): Uint8Array {
  const binaryData = atob(dataUrl.split(',')[1]);
  const bytes = new Uint8Array(binaryData.length);
  for (let i = 0; i < binaryData.length; i++) {
    bytes[i] = binaryData.charCodeAt(i);
  }
  return bytes;
}

// SSID 可能含 Windows 文件名非法字符（含控制/零宽字符），下载会静默失败，这里过滤
function sanitizeFileName(name: string): string {
  const cleaned = name
      .replace(/[<>:"/\\|?*\u0000-\u001F\u200B-\u200D\uFEFF]/g, '_')
      .replace(/[ .]+$/, '')
      .trim();
  return cleaned || 'WiFi';
}

// 在资源管理器中打开保存位置并选中文件
// downloadDir / join 走 core:path:default 已含的 resolve_directory + join，revealItemInDir 走 opener:default，无需扩权
// 若同名被浏览器追加序号（如 "xxx (1).png"），/select 失配时仍会打开所在文件夹
const revealSavedFile = async (fileName: string) => {
  try {
    await revealItemInDir(await join(await downloadDir(), fileName));
  } catch (err) {
    console.error('reveal saved file failed:', err);
    showNotification(t('notifyRevealFailed'), 'error');
  }
}

// 统一的本地保存通道：blob → <a download> → 延迟释放 blob URL → 成功 toast 带「打开文件夹」
// WebView2 对 <a download> + data: URL 的支持不稳定，二维码与 CSV 导出均走此路径
const saveBlobWithReveal = (fileName: string, blob: Blob, savedMessage: string) => {
  const blobUrl = URL.createObjectURL(blob);
  const downloadLink = document.createElement('a');
  downloadLink.href = blobUrl;
  downloadLink.download = fileName;
  downloadLink.style.display = 'none';
  document.body.appendChild(downloadLink);
  downloadLink.click();
  document.body.removeChild(downloadLink);
  // 同步 revoke 会掐断刚启动的下载，延迟释放
  setTimeout(() => URL.revokeObjectURL(blobUrl), 3000);
  showNotification(savedMessage, 'success', {
      label: t('openFolder'),
      run: () => void revealSavedFile(fileName),
  });
}

// Download QR code function
const downloadQRCode = (imgElement?: HTMLImageElement) => {
  const dataUrl = imgElement?.src || selectedPassword.value?.src || '';
  if (!dataUrl.startsWith('data:image/')) {
    showNotification(t('notifyNoQr'), 'error');
    return;
  }
  try {
    const fileName = `${sanitizeFileName(selectedPassword.value?.name ?? 'WiFi')}-WiFi-${locale.value === 'zh' ? '密码' : 'password'}.png`;
    saveBlobWithReveal(fileName, new Blob([dataUrlToUint8Array(dataUrl)], { type: 'image/png' }), t('notifyQrSaved'));
  } catch (err) {
    console.error('save qr failed:', err);
    showNotification(t('notifyQrSaveFailed'), 'error');
  }
}

// 导出已保存的 WiFi 配置为 CSV（纯本地生成 + 浏览器下载，与二维码保存同一机制）
function escapeCsvField(value: string): string {
  return `"${(value ?? '').replace(/"/g, '""')}"`;
}

const exportWifiCsv = () => {
  if (wifiList.value.length === 0) {
    showNotification(t('notifyExportEmpty'), 'error');
    return;
  }
  try {
    const header = [t('nameLabel'), t('passwordLabel'), t('backupColAuth'), t('backupColHidden')];
    const rows = wifiList.value.map(w => [
      w.name,
      w.password || t('noPassword'),
      w.auth,
      w.hidden ? t('yes') : t('no'),
    ]);
    // \uFEFF BOM：保证 Excel 正确识别 UTF-8 中文；\r\n 行尾兼容记事本/Excel
    const csv = '\uFEFF' + [header, ...rows]
        .map(row => row.map(escapeCsvField).join(','))
        .join('\r\n');
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8' });
    // 与二维码保存共用同一通道（延迟 revoke + 可打开所在文件夹）
    saveBlobWithReveal(
        `wifi-backup-${new Date().toISOString().slice(0, 10)}.csv`,
        blob,
        t('notifyExported'),
    );
  } catch (err) {
    console.error('export csv failed:', err);
    showNotification(t('notifyExportFailed'), 'error');
  }
}
// Copy QR code to clipboard
const copyQRCode = async () => {
  if (!selectedPassword.value || !selectedPassword.value.src) {
    showNotification(t('notifyNoQr'), 'error');
    return;
  }

  try {
    // data URL → 二进制 → PNG Blob
    const blob = new Blob([dataUrlToUint8Array(selectedPassword.value.src)], { type: 'image/png' });

    // Create a ClipboardItem and write to clipboard
    const item = new ClipboardItem({ 'image/png': blob });
    await navigator.clipboard.write([item]);

    showNotification(`${selectedPassword.value.name} ${t('notifyQrCopied')}`);
  } catch (err) {
    console.error('copy qr failed:', err);
    showNotification(t('notifyQrCopyFailed'), 'error');
  }
}

// ============ WiFi 二维码 payload 规范（WIFI: 配置，MeCard 式转义） ============
// SSID/密码中的 \ ; , : " 必须用反斜杠转义，否则扫码端解析截断
function escapeWifiQrField(value: string): string {
  return value.replace(/[\\;:,"]/g, '\\$&');
}

// 根据 netsh 身份验证字段映射到 WIFI: 规范的 T 值：nopass / WEP / WPA
// （WPA、WPA2、WPA3-SAE、混合模式统一用 WPA，手机扫码端会自行协商）
function detectSecurityType(wifi: Wifi): 'nopass' | 'WEP' | 'WPA' {
  if (!wifi.password) return 'nopass';
  const a = (wifi.auth || '').toUpperCase();
  if (a.includes('WEP')) return 'WEP';
  return 'WPA';
}

function buildWifiQrPayload(wifi: Wifi): string {
  const t = detectSecurityType(wifi);
  const parts = ['WIFI:'];
  parts.push(`T:${t};`);
  if (t !== 'nopass') parts.push(`P:${escapeWifiQrField(wifi.password)};`);
  parts.push(`S:${escapeWifiQrField(wifi.name)};`);
  if (wifi.hidden) parts.push('H:true;');
  return parts.join('');
}

// 修复 generateQRCode 计算属性
const generateQRCode = computed(() => {
  if (selectedPassword.value) {
    if (selectedPassword.value.src) {
      return selectedPassword.value.src;
    }
    const payload = buildWifiQrPayload(selectedPassword.value);
    QRCode.toDataURL(payload, { width: 400, margin: 2, errorCorrectionLevel: 'M' }).then(url => {
      const index = wifiList.value.findIndex(item => item.id === selectedPassword.value?.id);
      if (index !== -1) {
        wifiList.value[index].src = url;
      }
    });
  }
  return '';
})

// Clear search function
const clearSearch = () => {
  searchKey.value = ''
}

// Select a password to view details
const selectPassword = (password:Wifi) => {
  selectedPassword.value = password
  editedPassword.value = { ...password }
  showPassword.value = false
}

// Sidebar resize — Pointer Events + setPointerCapture + rAF 节流 + delta 计算
const startResize = (e: PointerEvent) => {
  if (isResizing.value) return;
  const target = e.currentTarget as HTMLElement;
  isResizing.value = true;
  activePointerId = e.pointerId;
  resizeStartX = e.clientX;
  resizeStartWidth = sidebarWidth.value;
  pendingWidth = null;
  try { target.setPointerCapture(e.pointerId); } catch { /* ignore */ }
  document.body.style.userSelect = 'none';
  document.body.style.cursor = 'col-resize';
}

const applyResizeFrame = () => {
  resizeRafId = 0;
  if (pendingWidth === null) return;
  // 基于起始宽度的 delta 保证越界时不“冻结”，回到区间也不会回弹跳变
  const clamped = Math.min(SIDEBAR_MAX, Math.max(SIDEBAR_MIN, pendingWidth));
  if (clamped !== sidebarWidth.value) sidebarWidth.value = clamped;
}

const onResizeMove = (e: PointerEvent) => {
  if (!isResizing.value) return;
  if (activePointerId !== null && e.pointerId !== activePointerId) return;
  // 不直接写 sidebarWidth，只记录目标值，下一帧统一 flush
  pendingWidth = resizeStartWidth + (e.clientX - resizeStartX);
  if (!resizeRafId) resizeRafId = requestAnimationFrame(applyResizeFrame);
}

const endResize = (e?: PointerEvent) => {
  if (!isResizing.value) return;
  isResizing.value = false;
  const target = e?.currentTarget as HTMLElement | undefined;
  if (target && activePointerId !== null && target.hasPointerCapture?.(activePointerId)) {
    try { target.releasePointerCapture(activePointerId); } catch { /* ignore */ }
  }
  if (resizeRafId) { cancelAnimationFrame(resizeRafId); resizeRafId = 0; }
  // flush 最后一帧，避免拖到边界时释放鼠标少一个像素
  applyResizeFrame();
  pendingWidth = null;
  activePointerId = null;
  document.body.style.userSelect = '';
  document.body.style.cursor = '';
  // Save sidebar width preference (v4 key: 默认宽度 288 → 336，旧键自动作废)
  localStorage.setItem('sidebarWidth.v4', sidebarWidth.value.toString());
}

// Refresh passwords function with animation
const refreshPasswords = async () => {
  if (isRefreshing.value) return;
  isRefreshing.value = true;
  searchKey.value = '';
  try{
    // await getPasswords();
    await getWifiPasswords();
  } catch (error){
    console.error('refresh failed:', error);
    showNotification(t('notifyRefreshFailed'), 'error');
  }finally {
    // Add a small delay before hiding the overlay for better UX
    setTimeout(() => {
      isRefreshing.value = false;
    }, 300);
  }
};

// Add subtle animation when switching between passwords
watch(selectedPassword, () => {
  if (selectedPassword.value) {
    const detailsElement = document.querySelector('.password-details');
    if (detailsElement) {
      detailsElement.classList.add('animate-fade-in');
      setTimeout(() => {
        detailsElement.classList.remove('animate-fade-in');
      }, 300);
    }
  }
});

// 监听派生的暗色状态（含 system 模式下系统偏好变化），同步 <html> 上的 .dark class
watch(isDarkMode, (newValue) => {
  if (newValue) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
  syncTrayTheme(newValue);
});

// 持久化主题模式选择
watch(themeMode, (newValue) => {
  localStorage.setItem('theme', newValue);
});

// 同步语言/主题到系统托盘（纯前端 vite 预览无 Tauri 环境，失败忽略）
function syncTrayLocale(locale: Locale) {
  emitTauriEvent('tray:locale', { locale }).catch(() => {})
}
function syncTrayTheme(dark: boolean) {
  emitTauriEvent('tray:theme', { dark }).catch(() => {})
}

// Persist locale selection
watch(locale, (newValue) => {
  localStorage.setItem('locale', newValue);
  document.documentElement.setAttribute('lang', newValue === 'zh' ? 'zh-CN' : 'en');
  syncTrayLocale(newValue);
});

// Load user preferences on mount
onMounted(() => {
  // getPasswords();
  refreshPasswords();
  // 主题：优先读三态 'theme' 键，兼容旧 'darkMode' 布尔键；默认浅色
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light' || savedTheme === 'dark' || savedTheme === 'system') {
    themeMode.value = savedTheme;
  } else {
    themeMode.value = localStorage.getItem('darkMode') === 'true' ? 'dark' : 'light';
  }
  // 跟随系统：初始化并监听系统深色偏好变化
  const darkMq = window.matchMedia('(prefers-color-scheme: dark)');
  systemDark.value = darkMq.matches;
  darkMq.addEventListener('change', (e) => { systemDark.value = e.matches; });

  // Load saved locale, fallback to browser language detection
  const savedLocale = localStorage.getItem('locale');
  if (savedLocale === 'zh' || savedLocale === 'en') {
    locale.value = savedLocale;
  } else {
    const nav = (navigator.language || 'zh').toLowerCase();
    locale.value = nav.startsWith('zh') ? 'zh' : 'en';
  }
  document.documentElement.setAttribute('lang', locale.value === 'zh' ? 'zh-CN' : 'en');

  // Check for saved sidebar width (v4) 并夹紧到合法区间
  const savedWidth = localStorage.getItem('sidebarWidth.v4');
  if (savedWidth) {
    const w = parseInt(savedWidth, 10);
    if (Number.isFinite(w)) {
      sidebarWidth.value = Math.min(SIDEBAR_MAX, Math.max(SIDEBAR_MIN, w));
    }
  }

  // 托盘初始状态：watch 默认不立即执行，启动时主动同步一次已持久化的语言与主题
  syncTrayLocale(locale.value);
  syncTrayTheme(isDarkMode.value);

  // 升级：仅生产环境启动后做一次后台检查；发现新版弹确认窗（失败/被跳过版本均不打扰）；dev 无安装上下文，跳过
  if (import.meta.env.PROD) {
    runUpdateCheck(true);
  }
})

window.addEventListener('contextmenu', (e) => e.preventDefault(), false);
// 在生成环境关闭鼠标右键
if (import.meta.env.MODE === "production") {
  window.addEventListener("contextmenu", (e) => e.preventDefault(), false);
}


</script>

<style>

/* 全局禁止文本选中 */
* {
  -webkit-user-select: none;
  user-select: none;
  /* 顺带禁用长按弹出的系统菜单（Tauri/WebView 常见） */
  -webkit-touch-callout: none;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.animate-fade-in {
  animation: fadeIn 0.3s ease-out;
}

/* Updated 3D Flip Card Styles - Brutalism */
.flip-card {
  perspective: 1000px;
  cursor: pointer;
}

.flip-card-inner {
  position: relative;
  width: 100%;
  height: 100%;
  text-align: center;
  transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1);
  transform-style: preserve-3d;
  transform-origin: center center;
}

.flip-card:hover .flip-card-inner {
  transform: rotateX(180deg);
}

.flip-card-front, .flip-card-back {
  position: absolute;
  width: 100%;
  height: 100%;
  -webkit-backface-visibility: hidden;
  backface-visibility: hidden;
  border-radius: 0;
  box-shadow: 4px 4px 0 #000;
}

.dark .flip-card-front, .dark .flip-card-back {
  box-shadow: 4px 4px 0 #fff;
}

.flip-card-back {
  transform: rotateX(180deg);
}

/* Custom scrollbar - moved to tailwind.css */

/* Responsive adjustments */
@media (max-width: 768px) {
  .grid-cols-3 {
    grid-template-columns: repeat(1, minmax(0, 1fr));
  }

  .flip-card {
    height: 280px;
    margin-bottom: 1rem;
  }
}

/* Resizable sidebar styles */
.cursor-col-resize {
  cursor: col-resize;
}

/* Brutalism cursor pointer */
.cursor-pointer {
  cursor: pointer;
}

.cursor-pointer:active {
  transform: translate(2px, 2px);
}
</style>
