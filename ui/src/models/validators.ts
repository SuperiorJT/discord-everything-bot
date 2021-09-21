import * as yup from 'yup';

export const EmbedValidator = yup.object().shape({
  title: yup.string(),
  description: yup.string().max(2000),
  url: yup.string().url(),
  timestamp: yup.date(),
  color: yup.string(),
  footer: yup.object().shape({
    text: yup.string()
  }),
  image: yup.object().shape({
    text: yup.string()
  }),
  thumbnail: yup.object().shape({
    text: yup.string()
  }),
  author: yup.object().shape({
    name: yup.string(),
    url: yup.string().url()
  }),
  fields: yup.array().of(
    yup.object().shape({
      name: yup.string(),
      value: yup.string()
    })
  )
});
