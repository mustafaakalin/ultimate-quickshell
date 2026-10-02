vim.cmd('highlight clear')
vim.g.colors_name = 'ultimate'
vim.o.termguicolors = true
local p = {
  bg = '#0f1117', surface = '#171a22', surface2 = '#222632', fg = '#f2f3f7', muted = '#aeb4c2',
  accent = '#b9c7ff', red = '#ffb4ab', yellow = '#ffd18a', green = '#a8e6cf', cyan = '#9fd7ff', blue = '#9fb5ff', magenta = '#d8b4ff'
}
local h = vim.api.nvim_set_hl
h(0, 'Normal', { fg=p.fg, bg=p.bg }); h(0, 'NormalFloat', { fg=p.fg, bg=p.surface }); h(0, 'FloatBorder', { fg=p.accent, bg=p.surface })
h(0, 'CursorLine', { bg=p.surface }); h(0, 'CursorLineNr', { fg=p.accent, bold=true }); h(0, 'LineNr', { fg=p.muted })
h(0, 'Comment', { fg=p.muted, italic=true }); h(0, 'Keyword', { fg=p.magenta, bold=true }); h(0, 'Statement', { fg=p.magenta, bold=true })
h(0, 'Function', { fg=p.cyan, bold=true }); h(0, 'Type', { fg=p.blue }); h(0, 'String', { fg=p.green }); h(0, 'Constant', { fg=p.yellow })
h(0, 'Number', { fg=p.yellow }); h(0, 'Boolean', { fg=p.yellow }); h(0, 'Operator', { fg=p.accent }); h(0, 'Identifier', { fg=p.fg })
h(0, 'Visual', { bg=p.surface2 }); h(0, 'Search', { fg=p.bg, bg=p.accent }); h(0, 'IncSearch', { fg=p.bg, bg=p.yellow })
h(0, 'ErrorMsg', { fg=p.red, bold=true }); h(0, 'DiagnosticError', { fg=p.red }); h(0, 'DiagnosticWarn', { fg=p.yellow }); h(0, 'DiagnosticInfo', { fg=p.cyan }); h(0, 'DiagnosticHint', { fg=p.blue })
h(0, 'DiffAdd', { fg=p.green, bg=p.surface }); h(0, 'DiffDelete', { fg=p.red, bg=p.surface }); h(0, 'DiffChange', { fg=p.yellow, bg=p.surface })
h(0, '@variable', { fg=p.fg }); h(0, '@variable.builtin', { fg=p.magenta }); h(0, '@function', { fg=p.cyan, bold=true }); h(0, '@function.call', { fg=p.cyan }); h(0, '@type', { fg=p.blue }); h(0, '@string', { fg=p.green }); h(0, '@comment', { fg=p.muted, italic=true })
h(0, '@lsp.type.function', { fg=p.cyan }); h(0, '@lsp.type.variable', { fg=p.fg }); h(0, '@lsp.type.type', { fg=p.blue }); h(0, '@lsp.type.parameter', { fg=p.yellow })
for i,c in ipairs({'#171a22','#ffb4ab','#a8e6cf','#ffd18a','#9fb5ff','#d8b4ff','#9fd7ff','#f2f3f7','#5d6270','#ff8f86','#7ad6ad','#efbd68','#7895ff','#ba91e8','#70bfdc','#ffffff'}) do vim.g['terminal_color_'..(i-1)] = c end