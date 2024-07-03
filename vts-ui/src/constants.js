export const FirmwaresRouteName = 'firmwares'
export const AgreementFirmwaresRouteName = 'agreementFirmwares'
export const AgreementsRouteName = 'agreements'
export const VehicleLinkRouteName = 'vehicleLink'
export const CreateAgreementRouteName = 'createAgreement'
export const VehicleRouteName = 'vehicle'
export const InvoicesRouteName = 'invoices'
export const InvoiceRouteName = 'invoice'

export const monthIndexToName = (month) => {
  const names = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ]
  return names[month - 1]
}
