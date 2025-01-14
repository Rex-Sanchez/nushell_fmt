# Intergration with neovim


Just add this to your init.lua or a other file that gets required by init.lua



```nushell
local function my_formatter()
  local bufnr = vim.api.nvim_get_current_buf()
  local content = vim.api.nvim_buf_get_lines(bufnr, 0, -1, false)

  local buf = table.concat(content, '\n')

  local temp_file = vim.fn.tempname()
  local file = io.open(temp_file, 'w')

  if (file ~= nil) then
    file:write(buf)
    file:close()
  else
    print 'Error could not open temp file'
    return
  end

  local cmd = string.format('./.cargo/bin/nushell_fmt -f %s', temp_file)

  local formatted_content = vim.fn.system(cmd)

  if vim.v.shell_error == 0 then
    local lines = vim.split(formatted_content, '\n')
    vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, lines)
  else
    print("Formatting failed!")

    print(formatted_content)
  end
end


vim.api.nvim_create_autocmd("FileType", {
  pattern = "nu",
  callback = function()
    vim.api.nvim_create_user_command('FormatBuffer', my_formatter, { nargs = 0 })

    -- change this binding to what you want, you can overwrite the default formatter map if you want this mapping will only apply in .nu files

    vim.api.nvim_buf_set_keymap(0, 'n', '<leader>cf', '<cmd>FormatBuffer<CR>', { noremap = true, silent = true })
  :noh
  end,
})
```
