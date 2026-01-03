import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {ClinicCapabilitiesService, ClinicCapability} from '../../../api_services/clinic-capabilities-service';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatCard, MatCardContent, MatCardHeader} from '@angular/material/card';

type LoadState = 'loading' | 'loaded' | 'error';

@Component({
  selector: 'app-clinic-capabilities',
  imports: [
    GenericDataTableComponent,
    MatCard,
    MatCardHeader,
    MatCardContent
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
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];
  constructor(private clinicCapabilitiesService:ClinicCapabilitiesService){}

  ngOnInit(): void {
    this.load();
  }
  private load(){
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


}
