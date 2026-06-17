import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';


export interface DentistHmoServiceAuditMatrixResponse {
    start_date: string;
    end_date: string;

    hmos: DentistHmoAuditHmoColumn[];
    rows: DentistHmoAuditDentistRow[];

    grand_total_qty: number;
    grand_total_fee: number;
}

export interface DentistHmoAuditHmoColumn {
    hmo_id: number;
    hmo_short_name: string;
    hmo_long_name: string;
}

export interface DentistHmoAuditDentistRow {
    dentist_id: number;
    dentist_name: string;

    cells: DentistHmoAuditCell[];

    row_total_qty: number;
    row_total_fee: number;
}

export interface DentistHmoAuditCell {
    hmo_id: number;

    services: DentistHmoAuditServiceLine[];

    cell_total_qty: number;
    cell_total_fee: number;
}

export interface DentistHmoAuditServiceLine {
    dental_service_id: number;
    dental_service_name: string;

    qty: number;
    service_fee: number;
    total_fee: number;
}
@Injectable({
  providedIn: 'root',
})
export class DentistHmoServicesMatrixService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly apiUrl = environment.apiUrl;
    private readonly base = `${this.apiUrl}/api/dentists/claims_matrix`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    getDentistHmoServicesClaimsMatrix(start_date:string, end_date:string,): Observable<DentistHmoServiceAuditMatrixResponse> {
        return this.http.get<DentistHmoServiceAuditMatrixResponse>(`${this.base}`,{
            headers: this.authHeaders(),
            params: {
                start_date: start_date,
                end_date: end_date,
            }
        });
    }

}
