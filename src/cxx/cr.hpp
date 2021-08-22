#pragma once

#include "rust/cxx.h"
#include <memory>

#define CR_HOST
#include "cr/cr.h"

typedef void *void_ptr;

/**
* constructs an instance of struct plugin
* @return               return new created instance of struct cr_plugin
*/
std::unique_ptr<cr_plugin> plugin_new();

/**
* Loads and initialize the plugin.
* @param    pCtx            a context that will manage the plugin internal data and user data.
* @param    pFullPath       full path with filename to the loadable binary for the plugin or `NULL`.
* @return                   `true` in case of success, `false` otherwise.
*/
bool plugin_open(std::unique_ptr<cr_plugin> pCtx, rust::String pFullPath);

/**
 * This function will call the plugin `cr_main` function. 
 * It should be called as frequently as the core logic/application needs.
 * @param   pCtx            a context that will manage the plugin internal data and user data.
 * @param   pReloadCheck	do a disk check (stat()) to see if the dynamic library needs a reload.
 * @return
 *          -1 if a failure happened during an update;
 *          -2 if a failure happened during a load or unload;
 *          anything else is returned directly from the plugin `cr_main`.
 */
int plugin_update(std::unique_ptr<cr_plugin> pCtx, bool pReloadCheck);

/**
 * Cleanup internal states once the plugin is not required anymore.
 */
void plugin_close();