# TODO: Automatic osquery Installation on First Launch

## Current State Analysis
- The app currently shows an "Install Osquery" button on the Home page when osquery is not installed
- Users must manually click this button to install osquery before they can use the application
- The installation process is handled in `src-tauri/src/osquery/install.rs` with platform-specific implementations
- The app flow is: SignIn → AccountSetup (if needed) → Home (with osquery check)

## Proposed Changes

### 1. **Add First Launch Detection**
- **File**: `src-tauri/src/lib.rs`
- **Action**: Create a new Tauri command `is_first_launch()` that checks if this is the first time the app is running
- **Implementation**: 
  - Use a local file (e.g., `~/.klaayguard/first_launch_complete`) to track if osquery has been installed
  - Return boolean indicating if this is the first launch

### 2. **Create Automatic Installation Function**
- **File**: `src-tauri/src/lib.rs`
- **Action**: Create a new Tauri command `auto_install_osquery()` that:
  - Checks if osquery is already installed
  - If not installed, automatically runs the installation process
  - Shows a loading/progress indicator during installation
  - Handles installation errors gracefully
  - Marks first launch as complete on successful installation

### 3. **Modify App Initialization Flow**
- **File**: `src/App.tsx`
- **Action**: Add a new route `/setup` for the automatic installation process
- **Implementation**:
  - Create a new `Setup` component that handles the automatic installation
  - Show loading states and progress during installation
  - Handle errors with retry options
  - Automatically redirect to signin after successful installation

### 4. **Create Setup Component**
- **File**: `src/pages/Setup/Setup.tsx`
- **Action**: Create a new component that:
  - Shows a welcome message explaining what's happening
  - Displays installation progress with appropriate messaging
  - Handles different installation states (checking, installing, success, error)
  - Provides retry functionality if installation fails
  - Automatically proceeds to signin on success

### 5. **Update Authentication Context**
- **File**: `src/context/AuthContext.tsx`
- **Action**: Modify the authentication flow to:
  - Check for first launch status on app initialization
  - Redirect to setup if this is the first launch
  - Only proceed to signin after osquery is confirmed installed

### 6. **Add Installation Progress Tracking**
- **File**: `src-tauri/src/osquery/install.rs`
- **Action**: Enhance the installation functions to:
  - Provide progress callbacks during installation
  - Handle different installation stages (downloading, installing, configuring)
  - Return detailed error information for better user feedback

### 7. **Update Home Component**
- **File**: `src/pages/Dashboard/Home.tsx`
- **Action**: Remove the manual installation button and related logic since installation will be automatic

### 8. **Add Error Handling and Recovery**
- **Files**: Multiple
- **Action**: Implement comprehensive error handling:
  - Network connectivity issues during download
  - Permission issues during installation
  - Platform-specific installation failures
  - Provide clear error messages and recovery options

### 9. **Add Installation Status Persistence**
- **File**: `src-tauri/src/lib.rs`
- **Action**: Create functions to:
  - Store installation status in a local file
  - Track installation timestamps
  - Handle installation verification on subsequent launches

### 10. **Update Build Configuration**
- **File**: `src-tauri/tauri.conf.json`
- **Action**: Ensure proper permissions are configured for:
  - File system access for status tracking
  - Network access for osquery downloads
  - System installation privileges

## Implementation Priority
1. **High Priority**: First launch detection and automatic installation
2. **Medium Priority**: Progress tracking and error handling
3. **Low Priority**: Enhanced UI/UX for installation process

## Security Considerations
- Ensure installation files are downloaded from trusted sources
- Validate downloaded packages before installation
- Handle installation with appropriate system permissions
- Secure storage of installation status

## Expected User Flow After Implementation
1. User launches app for first time
2. App automatically detects first launch
3. App shows setup screen with installation progress
4. osquery installs automatically in background
5. On successful installation, user is redirected to signin screen
6. Normal authentication flow proceeds
7. Subsequent launches skip setup and go directly to signin/home

This plan will eliminate the manual installation step and provide a seamless first-time user experience while maintaining the security and reliability of the osquery installation process. 