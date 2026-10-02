local M = {}
function M.setup(opts)
  opts = opts or {}
  vim.o.termguicolors = true
  vim.cmd.colorscheme(opts.colorscheme or 'ultimate')
end
return M