import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {ClinicCapabilitiesService, ClinicCapability} from '../../../api_services/clinic-capabilities-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {MatDialog} from '@angular/material/dialog';
import {AddEditClinicCapability} from './add-edit-clinic-capability/add-edit-clinic-capability';

type LoadState = 'loading' | 'loaded' | 'error';

@Component({
  selector: 'app-clinic-capabilities',
  standalone: true,
  imports: [
    GenericDataTableComponent,
    MatCard,
    MatCardHeader,
    MatCardContent,
    MatCardSubtitle,
    MatCardTitle
  ],
  templateUrl: './clinic-capabilities.html',
  styleUrl: './clinic-capabilities.scss',
})
export class ClinicCapabilities implements OnInit{
  state = signal<LoadState>('loading');
  clinicCapabilities= signal<ClinicCapability[] |null>(null);
  errorMsg = signal<string | null>(null);
  private destroyRef = inject(DestroyRef);

  clinicCapabilitiesColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On', cellTemplateKey: 'datetime'},
  ];
  constructor(private clinicCapabilitiesService:ClinicCapabilitiesService){}

  private dialog = inject(MatDialog);

  ngOnInit(): void {
    this.load_clinic_capabilities();
  }
  private load_clinic_capabilities(){
    this.state.set('loading');
    this.errorMsg.set(null)
    this.clinicCapabilitiesService.getClinicCapabilities()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.clinicCapabilities.set(res.items);
          this.state.set('loaded');
        },
        error: (err) => {
          console.log("In clinic capabilities, in Load():", err);
          this.errorMsg.set("Failed to load clinic capabilities");
          this.state.set('error');
        }
      })
  }

  openEditDialog(row:any) {
    console.log("In openRoleRowDialog():", row);
    const ref = this.dialog.open(AddEditClinicCapability, {
      data: {
        id: row.id,
        name: row.name,
        last_modified_by: row.last_modified_by,
        last_modified_on: row.last_modified_on,

      },
      width: '720px',
      maxWidth: '95vw',
      disableClose: false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed with result:', result);
      if (!result) return;
      this.clinicCapabilitiesService.patchClinicCapability(row.id, result.payload).subscribe(
        (patched)=>{console.log(`In openEditDialog(), updated clinic capability with id ${patched.id}`);this.load_clinic_capabilities();},
        (err)=>{console.log(err);}
      )
    });
  }

 openNewDialog(){
    const ref = this.dialog.open(AddEditClinicCapability, {
      data:{},
      width: '720px',
      maxWidth: '95vw',
      disableClose : false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('In openNewDialog(), the dialog was closed with result:', result);
      if (!result) return;
      this.clinicCapabilitiesService.postClinicCapability( result.payload ).subscribe(
        (inserted)=>{console.log(`In openNewDialog(), inserted clinic capability with id ${inserted.id}`);this.load_clinic_capabilities();},
        (err)=>{console.log(err);}
      )
    })
 }

}
