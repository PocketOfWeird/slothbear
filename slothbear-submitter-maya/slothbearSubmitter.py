import pymel.core as pm

values = {'submitted':False}

def submit(*args):
    print values

def textChanged(name, text):
    values[name] = text

#    Create a window with a some fields for entering text.
#
window = pm.window(title='Slothbear Submitter', width=700)
pm.rowColumnLayout( numberOfColumns=2, columnAttach=(1, 'right', 0), columnWidth=[(1, 200), (2, 500)] )

pm.text( label='Path to Project Directory' )
path_project = pm.textField(placeholderText='Smith\\MyAnimation', textChangedCommand=lambda *args: textChanged('path_project', args[0]))

pm.text( label='Path to Scene' )
path_scene = pm.textField(placeholderText='Smith\\MyAnimation\\awesomesauce.mb', textChangedCommand=lambda *args: textChanged('path_scene', args[0]))

pm.text( label='Path to Output Image Directory' )
path_output = pm.textField(placeholderText='Smith\\MyAnimation\\images\\firstTry', textChangedCommand=lambda *args: textChanged('path_output', args[0]))

pm.text( label='Output Image Leading Text' )
output_file_name = pm.textField(placeholderText='awesome', textChangedCommand=lambda *args: textChanged('output_file_name', args[0]))

pm.text( label='Camera' )
camera = pm.textField(placeholderText='persp1;top;bottom', textChangedCommand=lambda *args: textChanged('camera', args[0]))

pm.text( label='Frames' )
frames = pm.textField(placeholderText='1-20;35-79;88-240', textChangedCommand=lambda *args: textChanged('frames', args[0]))

pm.text( label='Frame Width Pixels' )
frame_width = pm.textField(placeholderText='1920', textChangedCommand=lambda *args: textChanged('frame_width', args[0]))

pm.text( label='Frame Height Pixels' )
frame_height = pm.textField(placeholderText='1080', textChangedCommand=lambda *args: textChanged('frame_height', args[0]))

pm.text( label='RenderPal Username' )
rp_user = pm.textField(placeholderText='If you want notifications, enter your RP Username', textChangedCommand=lambda *args: textChanged('rp_user', args[0]))

btn = pm.button(label='Submit', command=submit)

pm.showWindow( window )

