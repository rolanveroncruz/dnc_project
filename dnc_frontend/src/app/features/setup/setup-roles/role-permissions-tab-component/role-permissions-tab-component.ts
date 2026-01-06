import {Component, inject, Input, OnInit, } from '@angular/core';
import {
  GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {
  ModifiedRolePermission, Role,
} from '../../../../api_services/roles-and-permissions-service';
import {DataObject } from '../../../../api_services/data-objects-service';
import {MatDialog} from '@angular/material/dialog';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {AddEditRolePermissionsDialogComponent} from '../add-edit-role-permissions/add-edit-role-permissions.dialog';

@Component({
  selector: 'app-role-permissions-tab-component',
  imports: [
    GenericDataTableComponent
  ],
  templateUrl: './role-permissions-tab-component.html',
  styleUrl: './role-permissions-tab-component.scss',
})
export class RolePermissionsTabComponent implements OnInit{
  @Input() roles: Role[] | null = null;
  @Input() data_objects: DataObject[] | null = null;

  constructor(){
  }

  ngOnInit(): void {
    console.log("In RolePermissionsTabComponent(), roles:", this.roles);
    console.log("In RolePermissionsTabComponent(), data_objects:", this.data_objects);
  }

  @Input() role_permissions: ModifiedRolePermission[] | null = null;
  role_permissionsColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'role_name', label: 'Role'},
    {key: 'object_name', label: 'Object'},
    {key: 'actions', label: 'Action', cellTemplateKey: 'chips'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On', cellTemplateKey: 'date'},
  ];

  private dialog = inject(MatDialog);
  openRolePermissionRowDialog(row:ModifiedRolePermission){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditRolePermissionsDialogComponent, {
      autoFocus: false,
      data:{
        mode: 'edit',
        row,
        roles:this.roles,
        objects: this.data_objects,
      },
      width: '720px',
      maxWidth: '95vw',
      maxHeight: '90vh',
      disableClose : true,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }
}
