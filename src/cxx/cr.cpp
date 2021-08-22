#include "cr.hpp"

std::unique_ptr<cr_plugin> plugin_new()
{
     return std::make_unique<cr_plugin>();
}

bool plugin_open(std::unique_ptr<cr_plugin> pCtx, rust::String pFullPath)
{
     auto _ctx_ptr = pCtx.get();
     if (pFullPath.empty() || !_ctx_ptr)
     {
          return false;
     }
     return cr_plugin_open(*_ctx_ptr, pFullPath.c_str());
}

int plugin_update(std::unique_ptr<cr_plugin> pCtx, bool pReloadCheck)
{
     auto _ctx_ptr = pCtx.get();
     if (!_ctx_ptr)
     {
          return -1;
     }
     return cr_plugin_update(*_ctx_ptr, pReloadCheck);
}

void plugin_close(std::unique_ptr<cr_plugin> pCtx)
{
     auto _ctx_ptr = pCtx.get();
     if (!_ctx_ptr)
     {
          return;
     }
     return cr_plugin_close(*_ctx_ptr);
}