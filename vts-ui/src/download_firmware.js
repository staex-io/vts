export const downloadFirmware = (identity, arch, firmware) => {
  const firmwareUrl = URL.createObjectURL(
    new Blob([new Uint8Array(firmware).buffer], { type: 'application/zip' }),
  )
  const link = document.createElement('a')
  link.href = firmwareUrl
  link.download = `${identity}.firmware.${arch}.zip`
  document.body.appendChild(link)
  link.dispatchEvent(
    new MouseEvent('click', {
      bubbles: true,
      cancelable: true,
      view: window,
    }),
  )
  document.body.removeChild(link)
}
