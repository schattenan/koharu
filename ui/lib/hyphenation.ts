import { HyphenationLanguage } from '@/types'

export type HyphenationLanguageOption = {
  value: HyphenationLanguage | 'none'
  labelKey: string
}

export const hyphenationLanguages: HyphenationLanguageOption[] = [
  { value: 'none', labelKey: 'render.hyphenation.none' },
  { value: 'de', labelKey: 'render.hyphenation.de' },
  { value: 'en-us', labelKey: 'render.hyphenation.en-us' },
  { value: 'en-gb', labelKey: 'render.hyphenation.en-gb' },
  { value: 'fr', labelKey: 'render.hyphenation.fr' },
  { value: 'es', labelKey: 'render.hyphenation.es' },
  { value: 'it', labelKey: 'render.hyphenation.it' },
  { value: 'pt', labelKey: 'render.hyphenation.pt' },
  { value: 'nl', labelKey: 'render.hyphenation.nl' },
  { value: 'pl', labelKey: 'render.hyphenation.pl' },
  { value: 'ru', labelKey: 'render.hyphenation.ru' },
  { value: 'sv', labelKey: 'render.hyphenation.sv' },
  { value: 'da', labelKey: 'render.hyphenation.da' },
  { value: 'fi', labelKey: 'render.hyphenation.fi' },
  { value: 'cs', labelKey: 'render.hyphenation.cs' },
  { value: 'hu', labelKey: 'render.hyphenation.hu' },
  { value: 'tr', labelKey: 'render.hyphenation.tr' },
  { value: 'el', labelKey: 'render.hyphenation.el' },
  { value: 'uk', labelKey: 'render.hyphenation.uk' },
  { value: 'hr', labelKey: 'render.hyphenation.hr' },
  { value: 'ro', labelKey: 'render.hyphenation.ro' },
  { value: 'sk', labelKey: 'render.hyphenation.sk' },
  { value: 'sl', labelKey: 'render.hyphenation.sl' },
  { value: 'bg', labelKey: 'render.hyphenation.bg' },
  { value: 'ca', labelKey: 'render.hyphenation.ca' },
  { value: 'et', labelKey: 'render.hyphenation.et' },
  { value: 'lv', labelKey: 'render.hyphenation.lv' },
  { value: 'lt', labelKey: 'render.hyphenation.lt' },
  { value: 'id', labelKey: 'render.hyphenation.id' },
  { value: 'la', labelKey: 'render.hyphenation.la' },
]
