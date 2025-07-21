# Noted Mobile UI Specification

## Overview
This specification defines the mobile UI components, states, and interactions for Noted across Android (Jetpack Compose) and iOS (SwiftUI) platforms.

## Navigation Architecture

### Primary Navigation Flow
```
Entry Menu (Home)
    ↕ (vertical scroll)
Channels Screen ← → Notes Screen
                (horizontal scroll)
```

### Navigation Pattern
- **Gesture-based navigation** with scrolling between screens
- **Vertical scrolling**: Entry Menu ↔ Channels
- **Horizontal scrolling**: Channels ↔ Notes
- **No tab bar**: Full-screen immersive experience
- **Share Extension**: Opens channel selector when sharing from other apps

### Navigation Behavior
- **Entry Menu → Channels**: Scroll up
- **Channels → Entry Menu**: Scroll down
- **Channels → Notes**: 
  - Swipe left
  - Or tap channel to jump to another notes
- **Notes → Channels**: Swipe right
- **Selected Channel State**: Persists during navigation

## Screen Specifications

### 1. Entry Menu Screen

#### Purpose
Customizable dashboard for quick access to reminders, todos, AI search, and smart routing.

#### Components

##### Header
- **Title**: "Noted" (centered)
- **Search Button**: Global search icon (right)
- **Settings Button**: Gear icon (left)

##### Menu Grid
- **Layout**: Responsive grid (2-3 columns based on screen size)
- **Items**:
  - Reminders Widget (exportable to desktop)
  - Todo List Widget (exportable to desktop)
  - AI Search Card
  - Smart Route Card
  - Custom Action Cards (user-defined)
- **Behavior**: 
  - Long press to enter edit mode
  - Drag to reorder
  - Tap (+) to add new items
  - Widget items can be exported as desktop widgets

##### Bottom Input Bar
- **Quick Capture Input**: 
  - Placeholder: "Capture a thought..."
  - Mic button (right) for voice input
  - Camera button (left) for photo capture
- **AI Routing**: Automatically suggests channel on input

##### Floating Action Button
- **Position**: Bottom right
- **Action**: Opens expanded capture modal
- **Sub-actions** (on long press):
  - New Note
  - New Task
  - New Reminder
  - Voice Note
  - Photo Capture

#### States
```
EntryMenuState {
  menuItems: List<MenuItem>
  isEditMode: Boolean
  quickCaptureText: String
  suggestedChannel: Channel?
  isLoading: Boolean
  widgetExportable: Set<MenuItemId>
  locale: Locale
}

MenuItem {
  id: String
  type: MenuItemType
  title: LocalizedString
  icon: IconData
  isWidgetCompatible: Boolean
  widgetSizes: List<WidgetSize>?
}
```

### 2. Channels Screen

#### Purpose
Hierarchical view of all channels and categories with folder-like organization.

#### Components

##### Header
- **Title**: "Channels"
- **Search Button**: Channel search (right)
- **Add Button**: Create new channel/category (right)
- **Navigation Indicator**: Subtle down arrow or line indicating Entry Menu below

##### Channel List
- **Layout**: Vertical scrollable list
- **Item Types**:
  1. **Category Item**:
     - Expand/collapse chevron
     - Custom icon/image
     - Name
     - Note count badge
  2. **Channel Item**:
     - Custom icon/image
     - Name
     - Note count
     - Last updated timestamp
     - **Selection Indicator**: Highlight/border when selected

##### Navigation Hints
- **Pull-down indicator**: Shows Entry Menu is below
- **Selected Channel Indicator**: Visual feedback for current selection
- **Swipe Hint**: Subtle animation or arrow suggesting left swipe

##### Hierarchy Rules
- Maximum depth: 3 levels
- Categories can contain channels and sub-categories
- Channels cannot contain other items

##### Interactions
- **Tap Channel**: Select channel (visual feedback)
- **Swipe Left**: Navigate to selected channel's notes
- **Double Tap**: Select and immediately navigate to notes
- **Tap Category**: Expand/collapse
- **Long Press**: Enter reorder mode
- **Pull Down**: Return to Entry Menu

#### States
```
ChannelsState {
  channels: List<ChannelNode>
  selectedChannel: Channel?
  expandedCategories: Set<CategoryId>
  searchQuery: String
  isReorderMode: Boolean
  scrollPosition: Float
  isTransitioning: Boolean
}

ChannelNode {
  id: String
  name: String
  icon: IconData?
  customImage: ImageData?
  noteCount: Int
  lastUpdated: Timestamp
  children: List<ChannelNode>?
  depth: Int
}
```

### 3. Notes Screen

#### Purpose
Display and manage notes within a specific channel.

#### Components

##### Header
- **Channel Name**: With icon
- **Navigation Indicator**: Subtle right arrow indicating Channels screen
- **Search Button**: In-channel search
- **Menu Button**: Channel settings (three dots)

##### Notes List
- **Layout Options**:
  1. **Card View**: Pinterest-style cards with preview
  2. **Chat View**: Telegram-style bubbles
- **Note Preview**:
  - Title (if exists)
  - Text preview (2-3 lines)
  - Attachment thumbnails
  - Timestamp
  - Tags (if any)

##### Navigation Hints
- **Edge Glow**: Visual hint on right edge for swipe
- **Elastic Scroll**: Bounce effect when pulling right

##### Bottom Input Bar
- **Markdown Input**:
  - Expandable text field
  - Formatting toolbar (bold, italic, link, etc.)
  - Attachment button
  - Send button
- **Quick Actions**: Above keyboard when focused

##### Note Actions (Long Press/Swipe)
- Share
- Move to channel
- Reorder (drag handle)
- Archive
- Delete
- Edit

#### States
```
NotesState {
  channelId: String
  channelName: String
  notes: List<Note>
  searchQuery: String
  isSelectionMode: Boolean
  selectedNotes: Set<NoteId>
  inputText: String
  attachments: List<Attachment>
  viewMode: ViewMode // CARD or CHAT
  scrollPosition: Float
  isTransitioning: Boolean
}

Note {
  id: String
  content: String
  attachments: List<Attachment>
  createdAt: Timestamp
  updatedAt: Timestamp
  tags: List<Tag>
  position: Int
}
```

## Cross-Screen Components

### Share Sheet Handler

#### Purpose
Handle incoming shared content from other apps with channel selection or automatic routing.

#### UI Flow
1. **Share Reception**: App receives shared content
2. **Channel Selection Dialog**: Modal overlay
3. **Processing**: Save to selected channel
4. **Confirmation**: Success feedback

#### Components

##### Share Dialog
- **Header**:
  - Title: "Save to Noted"
  - Close button (X)
  
- **Content Preview**:
  - Shared content type icon
  - Title/URL/Text preview (truncated)
  - Image thumbnail (if applicable)
  - File info (name, size, type)

- **Channel Selection**:
  - **Auto-Route Toggle**:
    - Label: "Use AI routing"
    - Toggle switch
    - Suggested channel chip (when enabled)
  
  - **Manual Selection List**:
    - Search bar
    - Recent channels (3-5 items)
    - Divider
    - All channels (hierarchical list)
    - "Create New Channel" option

- **Actions**:
  - Cancel button
  - Save button (primary)

#### States
```
ShareHandlerState {
  sharedContent: SharedContent
  selectedChannel: Channel?
  suggestedChannel: Channel?
  useAutoRouting: Boolean
  isProcessing: Boolean
  recentChannels: List<Channel>
  searchQuery: String
}

SharedContent {
  type: ContentType // TEXT, URL, IMAGE, FILE, MULTIPLE
  title: String?
  text: String?
  url: String?
  images: List<ImageData>?
  files: List<FileData>?
  sourceApp: String
}
```

#### Platform Implementation

##### Android
```kotlin
// AndroidManifest.xml intent filters
<intent-filter>
    <action android:name="android.intent.action.SEND" />
    <category android:name="android.intent.category.DEFAULT" />
    <data android:mimeType="text/*" />
    <data android:mimeType="image/*" />
</intent-filter>
```

##### iOS
```swift
// Info.plist share extension
// ShareViewController.swift for share sheet
```

#### Behavior
- **Quick Save**: Long press on channel for instant save
- **Multi-select**: When sharing multiple items
- **Smart Routing**: 
  - Analyzes content type and text
  - Suggests most relevant channel
  - Learns from user corrections
- **Offline Support**: Queue shares when offline

### Search Interface
- **Global Search** (Entry/Channels): 
  - Searches across all notes
  - Shows results grouped by channel
  - Recent searches
  - Search filters (date, attachments, tags)
- **Channel Search** (Notes):
  - Searches within current channel
  - Inline results highlighting

### Sync Indicator
- **Location**: Status bar or below header
- **States**:
  - Syncing (animated)
  - Synced (checkmark)
  - Offline (cloud with slash)
  - Conflict (warning icon)
- **Conflict Resolution**:
  - Banner notification
  - Tap to view conflict details
  - Manual resolution options

### Attachment Viewer
- **Image**: Full-screen gallery with zoom
- **PDF**: Native PDF viewer
- **Video/Audio**: Native player controls
- **Documents**: Preview or open in external app

## Platform-Specific Considerations

### Android (Jetpack Compose)
- **Theme**: Material You with dynamic colors
- **Navigation**: Gesture-based with overscroll effects
- **Gestures**: 
  - Vertical/horizontal scroll detection
  - Velocity-based transitions
  - Edge-to-edge design
- **Back**: System back follows navigation hierarchy
- **Widgets**: Home screen widgets for quick capture
- **Share**: Intent filter for receiving shared content
- **Quick Settings**: Tile for quick capture

### iOS (SwiftUI)
- **Theme**: iOS design language with SF Symbols
- **Navigation**: Natural scrolling with haptic feedback
- **Gestures**: 
  - ScrollView with paging behavior
  - Rubber band scrolling
  - Gesture recognizers for navigation
- **Back**: Maintains iOS gesture conventions
- **Widgets**: iOS widgets and shortcuts
- **Share**: Share Extension for receiving content
- **Shortcuts**: Siri shortcuts for quick actions

## Animations & Transitions

### Screen Transitions
- **Push/Pop**: Slide horizontally with parallax
- **Modal**: Slide up from bottom
- **Tab Switch**: Fade with subtle scale

### List Animations
- **Item Addition**: Fade in with slide
- **Item Removal**: Fade out with slide
- **Reorder**: Smooth position changes
- **Expand/Collapse**: Height animation with fade

### Micro-interactions
- **Button Press**: Scale and haptic feedback
- **Toggle**: Smooth state change
- **Loading**: Skeleton screens
- **Pull to Refresh**: Elastic bounce

## Accessibility

### Requirements
- **Screen Reader**: Full support with descriptive labels
- **Dynamic Type**: Responsive text sizing
- **Color Contrast**: WCAG AA compliance
- **Keyboard Navigation**: Full support on tablets
- **Reduced Motion**: Respect system settings

### Focus Management
- Logical tab order
- Focus traps for modals
- Announce state changes

## Performance Considerations

### List Optimization
- **Lazy Loading**: Load items as needed
- **Recycling**: Reuse views in lists
- **Image Caching**: Progressive loading
- **Search Debouncing**: 300ms delay

### State Management
- **Local First**: Immediate UI updates
- **Background Sync**: Non-blocking
- **Pagination**: Load notes in chunks
- **Memory Management**: Clear unused caches

## Error Handling

### Network Errors
- Offline mode indication
- Retry mechanisms
- Graceful degradation

### User Errors
- Input validation
- Helpful error messages
- Undo/redo support

## Dark Mode

### Color Palette
- **Background**: Pure black (OLED) option
- **Surface**: Elevated grays
- **Primary**: Adaptive based on platform
- **Error**: Muted reds
- **Success**: Muted greens

### Contrast
- Maintain readability
- Adjust image overlays
- Dim attachment previews

## Internationalization (i18n)

### Supported Languages
- **Initial**: English, Traditional Chinese, Simplified Chinese
- **Planned**: Japanese, Korean, Spanish, French, German

### Text Direction
- **LTR**: Default for most languages
- **RTL**: Arabic, Hebrew support planned

### Implementation

#### String Resources
```
// Android: strings.xml
<string name="app_name">Noted</string>
<string name="capture_hint">Capture a thought...</string>
<string name="channels_title">Channels</string>

// iOS: Localizable.strings
"app_name" = "Noted";
"capture_hint" = "Capture a thought...";
"channels_title" = "Channels";
```

#### Dynamic Content
- **Date/Time**: Locale-specific formatting
- **Numbers**: Locale-specific separators
- **Plurals**: Language-specific plural rules

#### UI Considerations
- **Text Expansion**: Allow 30% extra space
- **Icon Labels**: Localized tooltips
- **Error Messages**: Context-aware translations

### Localization State
```
LocalizationState {
  currentLocale: Locale
  supportedLocales: List<Locale>
  textDirection: TextDirection
  dateFormat: DateFormat
  numberFormat: NumberFormat
}
```

## Desktop Widgets

### Widget Types

#### Android Widgets
- **Quick Capture Widget**
  - Sizes: 2x1, 4x1
  - Direct note input
  - Voice capture button
  - Opens specific channel
  
- **Todo List Widget**
  - Sizes: 2x2, 4x2, 4x4
  - Checkable items
  - Add new todo
  - Sync indicator

- **Recent Notes Widget**
  - Sizes: 3x3, 4x4
  - Scrollable list
  - Tap to open note
  - Channel indicators

- **Channel Shortcut Widget**
  - Size: 1x1
  - Custom icon
  - Direct channel access
  - Note count badge

#### iOS Widgets
- **Small Widget** (2x2)
  - Quick capture button
  - Last note preview
  - Channel shortcut

- **Medium Widget** (4x2)
  - Todo list view
  - Recent notes
  - Multi-channel view

- **Large Widget** (4x4)
  - Combined dashboard
  - Multiple entry menu items
  - Interactive elements

### Widget Features

#### Configuration
```
WidgetConfig {
  widgetId: String
  type: WidgetType
  targetChannel: Channel?
  displayMode: DisplayMode
  refreshInterval: Duration
  theme: WidgetTheme
}
```

#### Interactions
- **Tap Actions**: Open app, create note, open channel
- **Long Press**: Widget configuration
- **Refresh**: Pull latest data
- **Deep Links**: Direct navigation

#### Platform Integration
- **Android**:
  - App Widget API
  - Glance for Compose
  - Material You theming
  - Responsive sizing

- **iOS**:
  - WidgetKit
  - SwiftUI views
  - Widget intents
  - Smart stacks support

### Widget State Sync
```
WidgetSyncState {
  lastUpdated: Timestamp
  pendingNotes: List<Note>
  syncStatus: SyncStatus
  cachedData: WidgetCache
}
```

## Future Considerations

### Planned Features
- **Collaboration**: Real-time editing indicators
- **Rich Embeds**: Link previews
- **Voice Notes**: Waveform visualization
- **Templates**: Quick note templates
- **Batch Operations**: Multi-select actions
- **Widget Gallery**: More widget types
- **Complications**: Watch OS support

### Extensibility
- Plugin system for custom widgets
- Theming engine
- Export/import formats
- API for third-party apps
- Widget marketplace