export default () => ({
  menu(uuid) {
    let menu = document.getElementById(`menu-${uuid}`)
    menu.show()
  },
  truncate(hostname, l) {
    return hostname.length > l ? hostname.substring(0, l) + "..." : hostname
  }
})
