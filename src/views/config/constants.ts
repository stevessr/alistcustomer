export const defaultColors = {
  general: '#18a058',
  database: '#2080f0', 
  meilisearch: '#18a058',
  scheme: '#f0a020',
  log: '#d03050',
  task: '#9c27b0',
  cors: '#00bcd4',
  s3: '#ff9800',
  ftp: '#4caf50',
  sftp: '#3f51b5'
};

export const initializedConfig = {
  force: false,
  site_url: '',
  cdn: '',
  jwt_secret: '',
  token_expires_in: 0,
  section_colors: {},
  database: {
    type: 'mysql',
    host: '',
    port: 0,
    user: '',
    password: '',
    name: '',
    dbFile: '',
    tablePrefix: '',
    sslMode: 'disable',
    dsn: ''
  },
  meilisearch: {
    host: '',
    api_key: '',
    index_prefix: ''
  },
  scheme: {
    address: '',
    http_port: 0,
    https_port: 0,
    force_https: false,
    cert_file: '',
    key_file: '',
    unix_file: '',
    unix_file_perm: ''
  },
  temp_dir: '',
  bleve_dir: '',
  dist_dir: '',
  log: {
    enable: false,
    name: '',
    max_size: 0,
    max_backups: 0,
    max_age: 0,
    compress: false
  },
  delayed_start: 0,
  max_connections: 0,
  tls_insecure_skip_verify: false,
  tasks: {
    download: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    },
    transfer: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    },
    upload: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    },
    copy: {
      workers: 0,
      max_retry: 0,
      task_persistant: false
    }
  },
  cors: {
    allow_origins: [],
    allow_methods: [],
    allow_headers: []
  },
  s3: {
    enable: false,
    port: 0,
    ssl: false
  },
  ftp: {
    enable: false,
    listen: '',
    find_pasv_port_attempts: 0,
    active_transfer_port_non_20: false,
    idle_timeout: 0,
    connection_timeout: 0,
    disable_active_mode: false,
    default_transfer_binary: false,
    enable_active_conn_ip_check: false,
    enable_pasv_conn_ip_check: false
  },
  sftp: {
    enable: false,
    listen: ''
  }
};
