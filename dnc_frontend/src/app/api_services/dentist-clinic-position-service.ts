import { inject, Injectable } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

/**
 * Matches your SeaORM `position::Model`.
 * Adjust fields if your Position model has more columns.
 */
export interface DentistClinicPosition {
    id: number;
    name: string; // or `position_name`, etc. — align with your backend JSON
}

@Injectable({ providedIn: 'root' })
export class DentistClinicPositionService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }
    // If your backend is proxied via Angular dev proxy, you can set apiUrl = '' in environment.
    private readonly baseUrl = `${environment.apiUrl}/api`;

    /**
     * GET /api/dentist-clinic-positions
     */
    getAllPositions(): Observable<DentistClinicPosition[]> {
        return this.http.get<DentistClinicPosition[]>(
            `${this.baseUrl}/dentist_clinics/positions`,{headers: this.authHeaders()}
        );
    }
}
